// Copyright (c) 2022 Tony Barbitta
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::util::variadic::ZeroOrMore;

/// Makes a call to `puffin::set_scopes_on(true)` to begin `puffin` profiling, and starts the `puffin_http` server.
#[cfg(feature = "puffin")]
pub fn start_puffin_server() {
    puffin::set_scopes_on(true); // tell puffin to collect data

    match puffin_http::Server::new("0.0.0.0:8585") {
        Ok(puffin_server) => {
            eprintln!("Run:  cargo install puffin_viewer && puffin_viewer --url 127.0.0.1:8585");

            // We can store the server if we want, but in this case we just want
            // it to keep running. Dropping it closes the server, so let's not drop it!
            #[allow(clippy::mem_forget)]
            std::mem::forget(puffin_server);
        }
        Err(err) => {
            eprintln!("Failed to start puffin server: {}", err);
        }
    };
}

/// Set up the global `tracing_subscriber::FormatSubscriber` for the `tracing` crate.
///
/// ## Panics
/// - Panics if `tracing::subscriber::set_global_default` fails
#[cfg(feature = "trace")]
#[must_use]
pub fn setup_global_subscriber(bin_name: &str) -> impl Drop {
    use std::{fs::File, io::BufWriter};
    use tracing_flame::FlameLayer;
    use tracing_subscriber::{fmt, prelude::*, registry::Registry};

    let file_appender = tracing_appender::rolling::daily("./logs", format!("{}.log", bin_name));
    let (non_blocking, appender_guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().json().with_writer(non_blocking);

    let console_layer = fmt::layer().pretty().with_ansi(true);

    let (flame_layer, flame_guard) =
        FlameLayer::with_file(format!("./flames/{}.flame.folded", bin_name)).unwrap();
    let subscriber = Registry::default()
        .with(flame_layer)
        .with(file_layer)
        .with(console_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Could not set global default");

    MultiDrop::create(vec![
        Box::new(appender_guard) as Box<dyn Drop>,
        Box::new(flame_guard) as Box<dyn Drop>,
    ])
}

pub fn init_profiling(bin_name: &str) {
    #[cfg(feature = "puffin")]
    start_puffin_server();

    #[cfg(feature = "trace")]
    setup_global_subscriber(bin_name);
}

/// No-op when the `profiling` feature is disabled.
#[cfg(not(feature = "puffin"))]
pub fn start_puffin_server() {}

#[cfg(not(feature = "trace"))]
pub fn setup_global_subscriber(_bin_name: &str) -> impl Drop {
    DoNothingDrop
}

struct DoNothingDrop;

impl Drop for DoNothingDrop {
    fn drop(&mut self) {}
}

struct MultiDrop(Vec<Box<dyn Drop>>);

impl MultiDrop {
    fn empty() -> Self {
        Self(Vec::new())
    }

    fn create(drops: impl Into<ZeroOrMore<Box<dyn Drop>>>) -> Self {
        let drops: ZeroOrMore<Box<dyn Drop>> = drops.into();

        Self(drops.into_values())
    }

    fn push(&mut self, drop: Box<dyn Drop>) {
        self.0.push(drop);
    }
}

impl From<Vec<Box<dyn Drop>>> for MultiDrop {
    fn from(drops: Vec<Box<dyn Drop>>) -> Self {
        Self::create(drops)
    }
}

impl Drop for MultiDrop {
    fn drop(&mut self) {
        for d in self.0.drain(..) {
            std::mem::drop(d);
        }
    }
}
