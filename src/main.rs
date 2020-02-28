//! Subatomic is a lightweight wrapper for the Protonmail web interface.

use appkit::prelude::*;

mod app;
mod messages;
mod menu;
mod mailbox;
mod window;

fn main() {
    App::new(
        "com.secretkeys.subatomic",
        app::Subatomic::default()
    ).run();
}
