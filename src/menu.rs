//! The top-level menu. This could realistically live anywhere, but it's long and unwieldy as a
//! single block of code so it's extracted here.

use cacao::appkit::App;
use cacao::appkit::menu::{Menu, MenuItem};

/// Installs the menu.
pub fn build_menu() -> Vec<Menu> {
    vec![
        Menu::new("", vec![
            MenuItem::About("Subatomic".into()),
            MenuItem::Separator,
            MenuItem::new("Preferences").key(","),
            MenuItem::Separator,
            MenuItem::Services,
            MenuItem::Separator,
            MenuItem::Hide,
            MenuItem::HideOthers,
            MenuItem::ShowAll,
            MenuItem::Separator,
            MenuItem::Quit
        ]),

        Menu::new("File", vec![
            MenuItem::new("New Window").key("n"),
            MenuItem::Separator,
            MenuItem::CloseWindow,
            MenuItem::Separator,
            MenuItem::new("Export as PDF"),
            MenuItem::new("Print")
        ]),

        Menu::new("Edit", vec![
            MenuItem::Undo,
            MenuItem::Redo,
            MenuItem::Separator,
            MenuItem::Cut,
            MenuItem::Copy,
            MenuItem::Paste,
            MenuItem::Separator,
            MenuItem::SelectAll
        ]),
        
        Menu::new("View", vec![
            MenuItem::EnterFullScreen
        ]),

        Menu::new("Window", vec![
            MenuItem::Minimize,
            MenuItem::Zoom,
            MenuItem::Separator,
            MenuItem::new("Bring All to Front")
        ]),

        Menu::new("Help", vec![])
    ]
}
