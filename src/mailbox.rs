//! Implements the web view wrapper layer for Subatomic. Handles injecting scripts and routing
//! messages around accordingly.

use serde::Deserialize;

use cacao::filesystem::{FileSelectPanel, FileSavePanel};
use cacao::user_notifications::{NotificationCenter, Notification};
use cacao::webview::{
    WebView, WebViewConfig, WebViewDelegate,
    NavigationAction, NavigationResponse, OpenPanelParameters,
    InjectAt, NavigationPolicy, NavigationResponsePolicy
};

use crate::messages::Message;

#[derive(Debug, Default, Deserialize)]
pub struct NewMessageNotification {
    pub title: String,
    pub body: String
}

#[derive(Default)]
pub struct Mailbox;

impl Mailbox {
    pub fn config() -> WebViewConfig {
        let mut config = WebViewConfig::default();
        config.add_user_script(include_str!("../scripts/styles.js"), InjectAt::Start, false);
        config.add_user_script(include_str!("../scripts/scripts.js"), InjectAt::Start, false);
        config.add_handler("notify");
        config.add_handler("updateTitle");
        config 
    }
}

impl WebViewDelegate for Mailbox {
    fn did_load(&self, view: WebView) {
        view.load_url("https://beta.protonmail.com/"); 
    }

    fn on_message(&self, title: &str, body: &str) {
        if title == "notify" {
            let msg: NewMessageNotification = serde_json::from_str(body).unwrap();
            let notification = Notification::new(&msg.title, &msg.body);
            NotificationCenter::notify(notification);
        }

        if title == "updateTitle" {
            Message::UpdateTitle(body.to_string()).dispatch();
        }
    }

    fn policy_for_navigation_action<F: Fn(NavigationPolicy)>(&self, action: NavigationAction, handler: F) {
        let url = action.request.url();
        
        handler(match url.starts_with("https://beta.protonmail.com/") || url.starts_with("blob:https://beta.protonmail.com/") {
            true => NavigationPolicy::Allow,
            false => NavigationPolicy::Cancel
        });
    }

    fn policy_for_navigation_response<F: Fn(NavigationResponsePolicy)>(&self, response: NavigationResponse, handler: F) {
        handler(match response.can_show_mime_type {
            true => NavigationResponsePolicy::Allow,
            false => NavigationResponsePolicy::BecomeDownload
        });
    }

    fn run_open_panel<F: Fn(Option<Vec<String>>) + 'static>(&self, _params: OpenPanelParameters, handler: F) {
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
        let mut panel = FileSavePanel::new();
        panel.set_can_create_directories(true);
        panel.set_suggested_filename(suggested_filename);
        panel.show(move |path| {
            handler(true, path);
        });
    }
}
