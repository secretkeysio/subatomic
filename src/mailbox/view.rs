//! Implements the web view wrapper layer for Subatomic. Handles injecting scripts and routing
//! messages around accordingly.

use serde::Deserialize;

use appkit::prelude::*;
use appkit::webview::action::{
    NavigationAction, NavigationPolicy,
    NavigationResponse, NavigationResponsePolicy,
    OpenPanelParameters
};
use appkit::webview::config::InjectAt;
use appkit::file_panel::{FileSelectPanel, FileSavePanel, ModalResponse};

use crate::app::Subatomic;
use crate::messages::Message;

#[derive(Debug, Default, Deserialize)]
pub struct NewMessageNotification {
    pub title: String,
    pub body: String
}

pub struct MailboxViewController {
    pub view: WebView,
    pub select_panel: FileSelectPanel
}

impl Default for MailboxViewController {
    fn default() -> Self {
        let mut panel = FileSelectPanel::default();
        panel.set_can_choose_files(true);

        let vc = MailboxViewController {
            view: WebView::default(),
            select_panel: panel
        };

        vc.view.configure(&vc);
        vc.view.add_user_script(include_str!("../../scripts/styles.js"), InjectAt::Start, false);
        vc.view.add_user_script(include_str!("../../scripts/scripts.js"), InjectAt::Start, false);
        vc.view.add_handler("notify");
        vc.view.add_handler("updateTitle");

        vc
    }
}

impl ViewWrapper for MailboxViewController {
    fn get_handle(&self) -> Option<appkit::ShareId<appkit::Object>> {
        self.view.get_handle()
    }
}

fn dispatch(message: Message) {
    App::<Subatomic, Message>::dispatch(message);
}

impl ViewController for MailboxViewController {
    fn did_load(&self) {
        self.view.load_url("https://beta.protonmail.com/");
    }
}

impl WebViewController for MailboxViewController {
    fn on_message(&self, title: &str, body: &str) {
        if title == "notify" {
            let msg: NewMessageNotification = serde_json::from_str(body).unwrap();
            let notification = Notification::new(&msg.title, &msg.body);
            NotificationCenter::notify(notification);
        }

        if title == "updateTitle" {
            dispatch(Message::UpdateTitle(body.to_string()));
        }
    }

    fn policy_for_navigation_action<F: Fn(NavigationPolicy)>(&self, action: NavigationAction, handler: F) {
        let url = action.request.url();
        println!("Requesting: {}", url);
        handler(match url.starts_with("https://beta.protonmail.com/") || url.starts_with("blob:https://beta.protonmail.com/") {
            true => NavigationPolicy::Allow,
            false => NavigationPolicy::Cancel
        });
    }

    fn policy_for_navigation_response<F: Fn(NavigationResponsePolicy)>(&self, response: NavigationResponse, handler: F) {
        println!("Can show MIME: {}", response.can_show_mime_type);
        handler(match response.can_show_mime_type {
            true => NavigationResponsePolicy::Allow,
            false => NavigationResponsePolicy::BecomeDownload
        });
    }

    fn run_open_panel<F: Fn(Option<Vec<String>>) + 'static>(&self, params: OpenPanelParameters, handler: F) {
        let mut panel = FileSelectPanel::new();
        panel.set_can_choose_files(true);
        panel.show(move |paths| {
            handler(match paths.len() > 0 {
                true => Some(paths),
                false => None
            });
        });
    }

    fn run_save_panel<F: Fn(bool, Option<String>) + 'static>(&self, suggested_filename: &str, handler: F) {
        println!("Suggested filename: {:?}", suggested_filename);
        let mut panel = FileSavePanel::new();
        panel.set_can_create_directories(true);
        panel.set_suggested_filename(suggested_filename);
        panel.show(move |path| {
            println!("...?");
            handler(true, path);
        });
    }
}
