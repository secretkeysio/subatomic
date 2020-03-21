//! Implements the basic app delegate.

use appkit::app::{AppDelegate, Dispatcher};
use appkit::layout::{Layout, LayoutConstraint};
use appkit::notifications::{NotificationCenter, NotificationAuthOption};
use appkit::view::View;
use appkit::webview::WebView;
use appkit::window::{Window, WindowConfig};

use crate::messages::Message;
use crate::menu::set_menu;
use crate::mailbox::Mailbox;

pub struct Subatomic {
    pub window: Window,
    pub view: View,
    pub webview: WebView<Mailbox>,
}

impl Default for Subatomic {
    fn default() -> Self {
        let window_config = WindowConfig::default();

        let webview_config = Mailbox::config();

        Subatomic {
            window: Window::new(window_config),
            view: View::new(),
            webview: WebView::with(webview_config, Mailbox::default())
        }
    }
}

impl AppDelegate for Subatomic {
    fn did_finish_launching(&self) {
        set_menu();

        NotificationCenter::request_authorization(&[
            NotificationAuthOption::Badge,
            NotificationAuthOption::Sound,
            NotificationAuthOption::Alert
        ]);
        
        self.window.set_autosave_name("com.subatomic.mailbox");
        self.window.set_titlebar_appears_transparent(true);
        self.window.set_movable_by_background(true);
        self.window.set_title("ProtonMail Login");

        // Need a backing layer in case the inspector opens, otherwise things go crazy ;P
        self.view.add_subview(&self.webview);
        self.window.set_content_view(&self.view);

        LayoutConstraint::activate(&[
            self.webview.top.constraint_equal_to(&self.view.top),
            self.webview.leading.constraint_equal_to(&self.view.leading),
            self.webview.trailing.constraint_equal_to(&self.view.trailing),
            self.webview.bottom.constraint_equal_to(&self.view.bottom)
        ]);

        self.window.show();
    }

    fn did_become_active(&self) {
        NotificationCenter::remove_all_delivered_notifications();
        self.window.show();
    }
}

impl Dispatcher for Subatomic {
    type Message = Message;

    fn on_message(&self, message: Self::Message) {
        match message {
            Message::UpdateTitle(s) => self.window.set_title(&s)
        }
    }
}
