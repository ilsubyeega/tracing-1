//! A simple example demonstrating how one might implement a custom
//! subscriber.
//!
//! This subscriber implements a tree-structured logger similar to
//! the "compact" formatter in [`slog-term`]. The demo mimics the
//! example output in the screenshot in the [`slog` README].
//!
//! Note that this logger isn't ready for actual production use.
//! Several corners were cut to make the example simple.
//!
//! [`slog-term`]: https://docs.rs/slog-term/2.4.0/slog_term/
//! [`slog` README]: https://github.com/slog-rs/slog#terminal-output-example
#![deny(rust_2018_idioms)]

use tracing::{debug, info, span, warn, Level};

mod hierarchical_subscriber;
use self::hierarchical_subscriber::HierarchicalSubscriber;

fn main() {
    let subscriber = HierarchicalSubscriber::new(2);
    tracing::subscriber::set_global_default(subscriber).unwrap();

    let app_span = span!(Level::TRACE, "hierarchical-example", version = %0.1);
    let _e = app_span.enter();

    let server_span = span!(Level::TRACE, "server", host = "localhost", port = 8080);
    let _e2 = server_span.enter();
    info!("starting");
    std::thread::sleep(std::time::Duration::from_millis(300));
    info!("listening");
    let peer1 = span!(Level::TRACE, "conn", peer_addr = "82.9.9.9", port = 42381);
    peer1.in_scope(|| {
        debug!("connected");
        std::thread::sleep(std::time::Duration::from_millis(300));
        debug!(length = 2, "message received");
    });
    let peer2 = span!(Level::TRACE, "conn", peer_addr = "8.8.8.8", port = 18230);
    peer2.in_scope(|| {
        std::thread::sleep(std::time::Duration::from_millis(300));
        debug!("connected");
    });
    peer1.in_scope(|| {
        warn!(algo = "xor", "weak encryption requested");
        std::thread::sleep(std::time::Duration::from_millis(300));
        debug!(length = 8, "response sent");
        debug!("disconnected");
    });
    peer2.in_scope(|| {
        debug!(length = 5, "message received");
        std::thread::sleep(std::time::Duration::from_millis(300));
        debug!(length = 8, "response sent");
        debug!("disconnected");
    });
    warn!("internal error");
    info!("exit");
}