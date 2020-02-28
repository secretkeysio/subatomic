//! Implements the Toolbar for the mailbox view. Notably, this isn't shown on the login screen -
//! we loop some messages around to ensure it's shown properly.

use appkit::button::Button;
use appkit::toolbar::{ToolbarDelegate, ToolbarItem};

#[derive(Default)]
pub struct MailboxToolbar {}

impl ToolbarDelegate for MailboxToolbar {
    fn allowed_item_identifiers(&self) -> Vec<&'static str> {
        vec!["test"]
    }

    fn default_item_identifiers(&self) -> Vec<&'static str> {
        vec!["test"]
    }

    fn item_for(&self, _identifier: &str) -> ToolbarItem {
        let mut item = ToolbarItem::new("test");
        item.set_title("DOES THIS WORK?!");

        let button = Button::new("Hello?");
        item.set_button(button);

        item
    }
}
