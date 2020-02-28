//! Messages that we used to thread control throughout the application. If you come from
//! React/Redux, you can liken it to that world.

//use appkit::app::Dispatchable;

#[derive(Debug)]
pub enum Message {
    UpdateTitle(String)
}

//impl Dispatchable for Message {}
