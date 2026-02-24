mod aggregates;
mod client;
mod db;
mod downloads;
mod goals;

use anyhow::Result;
use clap::{Parser, Subcommand};
use client::GitHubClient;
use db::init_db;
use goals::Config;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use octocrab::OctocrabBuilder;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::level_filters::LevelFilter;

const ORG: &str = "strands-agents";

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(long, short, default_value = "metrics.db")]
    db_path: PathBuf,
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Incremental sync of GitHub data.
    Sync,
    /// Garbage-collect items deleted upstream.
    Sweep,
    /// Run raw SQL.
    Query { sql: String },
    /// Load goals, team, and package mappings from config.toml.
    LoadConfig {
        #[clap(default_value = "strands-grafana/config.toml")]
        config_path: PathBuf,
    },
    /// List all configured goals.
    ListGoals,
    /// Sync package download stats from PyPI and npm.
    SyncDownloads {
        #[clap(long, default_value = "strands-grafana/config.toml")]
        config_path: PathBuf,
        /// Number of days to fetch
        #[clap(long, default_value = "30")]
        days: i64,
    },
    /// Backfill historical download data (PyPI ~180d, npm ~365d).
    BackfillDownloads {
        #[clap(long, default_value = "strands-grafana/config.toml")]
        config_path: PathBuf,
    },
}

fn create_spinner(m: &Arc<MultiProgress>, message: &str) -> ProgressBar {
    let sty = ProgressStyle::with_template("{spinner:.green} {msg}")
        .unwrap()
        .tick_chars("\u{280b}\u{2819}\u{2839}\u{2838}\u{283c}\u{2834}\u{2826}\u{2827}\u{2807}\u{280f} ");
    let pb = m.add(ProgressBar::new_spinner());
    pb.set_style(sty);
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
    pb.set_message(message.to_string());
    pb
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(LevelFilter::WARN)
        .init();

    let args = Cli::parse();
    let mut conn = init_db(&args.db_path)?;
    goals::init_goals_table(&conn)?;

    match args.command {
        Commands::Sync => {
            let gh_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
            let octocrab = OctocrabBuilder::new().personal_token(gh_token).build()?;

            let m = Arc::new(MultiProgress::new());
            let pb = create_spinner(&m, "Syncing...");

            let mut client = GitHubClient::new(octocrab, &mut conn, pb.clone());
            client.sync_org(ORG).await?;

            pb.set_message("Computing metrics...");
            aggregates::compute_metrics(&conn)?;

            pb.finish_with_message("Done.");
        }
        Commands::Sweep => {
            let gh_token = std::env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN must be set");
            let octocrab = OctocrabBuilder::new().personal_token(gh_token).build()?;

            let m = Arc::new(MultiProgress::new());
            let pb = create_spinner(&m, "Sweeping...");

            let mut client = GitHubClient::new(octocrab, &mut conn, pb.clone());
            client.sweep_org(ORG).await?;

            pb.finish_with_message("Sweep complete.");
        }
        Commands::Query { sql } => {
            let mut stmt = conn.prepare(&sql)?;
            let column_count = stmt.column_count();
            let names: Vec<String> = stmt.column_names().into_iter().map(String::from).collect();

            println!("{}", names.join(" | "));
            println!("{}", "-".repeat(names.len() * 15));

            let mut rows = stmt.query([])?;
            while let Some(row) = rows.next()? {
                let mut row_values = Vec::new();
                for i in 0..column_count {
                    let val = row.get_ref(i)?;
                    let text = match val {
                        rusqlite::types::ValueRef::Null => "NULL".to_string(),
                        rusqlite::types::ValueRef::Integer(i) => i.to_string(),
                        rusqlite::types::ValueRef::Real(f) => f.to_string(),
                        rusqlite::types::ValueRef::Text(t) => {
                            String::from_utf8_lossy(t).to_string()
                        }
                        rusqlite::types::ValueRef::Blob(_) => "<BLOB>".to_string(),
                    };
                    row_values.push(text);
                }
                println!("{}", row_values.join(" | "));
            }
        }
        Commands::LoadConfig { config_path } => {
            let config = Config::load(&config_path)?;

            let goal_count = goals::load_goals(&conn, &config)?;
            let team_count = goals::load_team(&conn, &config.members)?;
            let mapping_count = goals::load_repo_mappings(&conn, &config)?;

            println!(
                "Loaded {} goals, {} team members, {} package mappings from {}",
                goal_count,
                team_count,
                mapping_count,
                config_path.display()
            );
        }
        Commands::ListGoals => {
            let all_goals = goals::list_goals(&conn)?;
            println!(
                "{:<40} | {:>10} | {:<20} | {:<15} | Warning Ratio",
                "Metric", "Value", "Label", "Direction",
            );
            println!("{}", "-".repeat(110));
            for goal in all_goals {
                let label_str = goal.label.as_deref().unwrap_or("-");
                let ratio_str = goal
                    .warning_ratio
                    .map_or_else(|| "-".to_string(), |r| format!("{r:.2}"));
                println!(
                    "{:<40} | {:>10} | {:<20} | {:<15} | {}",
                    goal.metric, goal.value, label_str, goal.direction, ratio_str
                );
            }
        }
        Commands::SyncDownloads { config_path, days } => {
            let config = Config::load(&config_path)?;
            let mapping_count = goals::load_repo_mappings(&conn, &config)?;
            println!("Loaded {mapping_count} package mappings\n");

            println!("Syncing PyPI packages...");
            for package in config.packages_for_registry("pypi") {
                match downloads::sync_pypi_downloads(&conn, &package, days).await {
                    Ok(count) => println!("  {package} - {count} data points"),
                    Err(e) => eprintln!("  {package} - error: {e}"),
                }
            }

            println!("\nSyncing npm packages...");
            for package in config.packages_for_registry("npm") {
                match downloads::sync_npm_downloads(&conn, &package, days).await {
                    Ok(count) => println!("  {package} - {count} data points"),
                    Err(e) => eprintln!("  {package} - error: {e}"),
                }
            }

            println!("\nDone.");
        }
        Commands::BackfillDownloads { config_path } => {
            let config = Config::load(&config_path)?;
            let mapping_count = goals::load_repo_mappings(&conn, &config)?;
            println!("Loaded {mapping_count} package mappings\n");

            println!("Backfilling PyPI (up to 180 days)...");
            for package in config.packages_for_registry("pypi") {
                match downloads::backfill_pypi_downloads(&conn, &package).await {
                    Ok(count) => println!("  {package} - {count} data points"),
                    Err(e) => eprintln!("  {package} - error: {e}"),
                }
            }

            println!("\nBackfilling npm (up to 365 days)...");
            for package in config.packages_for_registry("npm") {
                match downloads::backfill_npm_downloads(&conn, &package).await {
                    Ok(count) => println!("  {package} - {count} data points"),
                    Err(e) => eprintln!("  {package} - error: {e}"),
                }
            }

            println!("\nDone.");
        }
    }

    Ok(())
}
