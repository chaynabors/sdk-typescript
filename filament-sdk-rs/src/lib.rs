#![allow(clippy::all)]

mod bindings {
    wit_bindgen::generate!({
        path: "../filament-wit/filament.wit",
        pub_export_macro: true,
        async: true,
    });
}

pub use bindings::*;
