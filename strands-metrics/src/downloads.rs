use anyhow::{Context, Result};
use chrono::{Duration, Utc};
use rusqlite::{params, Connection};
use serde::Deserialize;

const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

#[derive(Debug, Deserialize)]
struct PyPIStatsResponse {
    data: Vec<PyPIDataPoint>,
}

#[derive(Debug, Deserialize)]
struct PyPIDataPoint {
    date: String,
    downloads: i64,
}

#[derive(Debug, Deserialize)]
struct NpmRangeResponse {
    downloads: Vec<NpmDownloadPoint>,
}

#[derive(Debug, Deserialize)]
struct NpmDownloadPoint {
    day: String,
    downloads: i64,
}

pub async fn sync_pypi_downloads(conn: &Connection, package: &str, days: i64) -> Result<usize> {
    let client = reqwest::Client::new();

    let url = format!(
        "https://pypistats.org/api/packages/{}/overall?mirrors=false",
        package
    );

    let response: PyPIStatsResponse = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?
        .json()
        .await
        .with_context(|| format!("failed to fetch PyPI stats for {package}"))?;

    let cutoff = (Utc::now() - Duration::days(days))
        .format("%Y-%m-%d")
        .to_string();
    let max_date = (Utc::now() - Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();

    let mut count = 0;
    for point in response.data {
        if point.date >= cutoff && point.date <= max_date {
            conn.execute(
                "INSERT INTO package_downloads (date, package, registry, version, downloads)
                 VALUES (?1, ?2, 'pypi', 'total', ?3)
                 ON CONFLICT(date, package, registry, version) DO UPDATE SET downloads = excluded.downloads",
                params![point.date, package, point.downloads],
            )?;
            count += 1;
        }
    }

    Ok(count)
}

pub async fn sync_npm_downloads(conn: &Connection, package: &str, days: i64) -> Result<usize> {
    let client = reqwest::Client::new();

    let end_date = (Utc::now() - Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();
    let start_date = (Utc::now() - Duration::days(days))
        .format("%Y-%m-%d")
        .to_string();

    let url = format!(
        "https://api.npmjs.org/downloads/range/{}:{}/{}",
        start_date, end_date, package
    );

    let response: NpmRangeResponse = client
        .get(&url)
        .header("User-Agent", USER_AGENT)
        .send()
        .await?
        .json()
        .await
        .with_context(|| format!("failed to fetch npm stats for {package}"))?;

    let mut count = 0;
    for point in response.downloads {
        conn.execute(
            "INSERT INTO package_downloads (date, package, registry, version, downloads)
             VALUES (?1, ?2, 'npm', 'total', ?3)
             ON CONFLICT(date, package, registry, version) DO UPDATE SET downloads = excluded.downloads",
            params![point.day, package, point.downloads],
        )?;
        count += 1;
    }

    Ok(count)
}

pub async fn backfill_pypi_downloads(conn: &Connection, package: &str) -> Result<usize> {
    sync_pypi_downloads(conn, package, 180).await
}

pub async fn backfill_npm_downloads(conn: &Connection, package: &str) -> Result<usize> {
    sync_npm_downloads(conn, package, 365).await
}
