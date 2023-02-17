//! Subatomic is a lightweight wrapper for the Protonmail web interface.

use cacao::appkit::App;

mod app;
mod messages;
mod menu;
mod mailbox;

fn main() {
    App::new(
        "com.secretkeys.subatomic",
        app::Subatomic::default()
    ).run();
}
