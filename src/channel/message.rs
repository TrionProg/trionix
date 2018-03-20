
use std::fmt::Debug;
use std::any::Any;

use failure::Error;

use process::ProcessInfo;

pub type SignalCode=u32;
pub type MessageCode=u32;
pub type AnySignal=Box<Any+Send>;
pub type AnyMessage=Box<Any+Send>;

pub trait SignalTrait:Debug{ //TODO failure Error trait?
    fn code(&self) -> SignalCode;
}

pub trait MessageTrait:Debug{
    fn code(&self) -> MessageCode;
}

pub struct Message {
    pub from_process:ProcessInfo,
    pub content:MessageContent
}

impl Message {
    pub fn new(from_process:ProcessInfo, content:MessageContent) -> Self{
        Message {
            from_process,
            content
        }
    }
}

pub enum MessageContent {
    Signal(AnySignal),
    Message(AnyMessage)
}