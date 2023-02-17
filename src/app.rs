//! Implements the basic app delegate.

use cacao::appkit::{App, AppDelegate};
use cacao::appkit::window::{Window, WindowConfig};
use cacao::notification_center::Dispatcher;
use cacao::layout::{Layout, LayoutConstraint};
use cacao::user_notifications::{NotificationCenter, NotificationAuthOption};
use cacao::view::View;
use cacao::webview::WebView;

use crate::messages::Message;
use crate::menu::build_menu;
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
        App::set_menu(build_menu());
        App::activate();

        /*NotificationCenter::request_authorization(&[
            NotificationAuthOption::Badge,
            NotificationAuthOption::Sound,
            NotificationAuthOption::Alert
        ]);*/
        
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

        //self.window.show();
    }

    fn did_become_active(&self) {
        self.window.show();
        //NotificationCenter::remove_all_delivered_notifications();
        //self.window.show();
    }
}

impl Dispatcher for Subatomic {
    type Message = Message;

    fn on_ui_message(&self, message: Self::Message) {
        match message {
            Message::UpdateTitle(s) => self.window.set_title(&s)
        }
    }
}
