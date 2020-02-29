//! Wrapper for Window.

use appkit::prelude::*;

use crate::mailbox::MailboxViewController;

#[derive(Default, WindowWrapper)]
pub struct MailboxWindowController {
    pub window: Window,
    pub mailbox: MailboxViewController
}

impl WindowController for MailboxWindowController {
    fn autosave_name(&self) -> &str { "com.secretkeys.subatomic.mailbox" }

    fn did_load(&self) {
        self.window.set_titlebar_appears_transparent(true);
        self.window.set_movable_by_background(true);
        self.window.set_title("ProtonMail Login");
        self.window.set_content_view(&self.mailbox);
    }

    fn will_close(&self) {}
}
