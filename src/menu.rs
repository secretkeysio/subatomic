//! The top-level menu. This could realistically live anywhere, but it's long and unwieldy as a
//! single block of code so it's extracted here.

use cacao::macos::{
    app::App, menu::{Menu, MenuItem}
};

/// Installs the menu.
pub fn set_menu() {
    App::set_menu(vec![
        Menu::new("", vec![
            MenuItem::about("Subatomic"),
            MenuItem::Separator,
            MenuItem::action("Preferences").key(","),
            MenuItem::Separator,
            MenuItem::services(),
            MenuItem::Separator,
            MenuItem::hide(),
            MenuItem::hide_others(),
            MenuItem::show_all(),
            MenuItem::Separator,
            MenuItem::quit()
        ]),

        Menu::new("File", vec![
            MenuItem::action("New Window").key("n"),
            MenuItem::Separator,
            MenuItem::close_window(),
            MenuItem::Separator,
            MenuItem::action("Export as PDF"),
            MenuItem::action("Print")
        ]),

        Menu::new("Edit", vec![
            MenuItem::undo(),
            MenuItem::redo(),
            MenuItem::Separator,
            MenuItem::cut(),
            MenuItem::copy(),
            MenuItem::paste(),
            MenuItem::Separator,
            MenuItem::select_all()
        ]),
        
        Menu::new("View", vec![
            MenuItem::enter_full_screen()
        ]),

        Menu::new("Window", vec![
            MenuItem::minimize(),
            MenuItem::zoom(),
            MenuItem::Separator,
            MenuItem::action("Bring All to Front")
        ]),

        Menu::new("Help", vec![])
    ]);
}
