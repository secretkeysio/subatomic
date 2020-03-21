//! Messages that we used to thread control throughout the application. If you come from
//! React/Redux, you can liken it to that world.

use appkit::app::App;
use crate::app::Subatomic;

#[derive(Clone, Debug)]
pub enum Message {
    UpdateTitle(String)
}

impl Message {
    pub fn dispatch(self) {
        App::<Subatomic, Message>::dispatch(self);
    }
}
