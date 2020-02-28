//! Implements the basic app delegate.

use appkit::prelude::*;

use crate::messages::Message;
use crate::menu::set_menu;
use crate::window::MailboxWindowController;

#[derive(Default)]
pub struct Subatomic {
    pub window: MailboxWindowController
}

impl AppDelegate for Subatomic {
    type Message = Message;

    fn on_message(&self, message: Self::Message) {
        match message {
            Message::UpdateTitle(s) => self.window.set_title(&s)
        }
    }

    fn did_finish_launching(&self) {
        set_menu();

        NotificationCenter::request_authorization(
            NotificationAuthOption::Badge | NotificationAuthOption::Sound | NotificationAuthOption::Alert
        );

        self.window.show();
    }

    fn did_become_active(&self) {
        NotificationCenter::remove_all_delivered_notifications();
        self.window.show();
    }
}
