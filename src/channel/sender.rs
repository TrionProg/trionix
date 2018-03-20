
use std::sync::mpsc;

use failure::Error;
use failure::err_msg;

use process::ProcessInfo;

use errors::BrokenChannel;

use channel::{Message, MessageContent};
use channel::{AnySignal,AnyMessage};

pub struct Sender {
    from_process:ProcessInfo,
    to_process:ProcessInfo,
    remote:bool,
    sender:mpsc::Sender< Message >,
}

pub enum SendResult {
    Ok,
    Timeout,
    Error(Error)
}

impl Sender {
    pub fn new(to_process:ProcessInfo, sender:mpsc::Sender< Message >) -> Self {
        let remote=to_process.process_id.is_local();

        Sender {
            from_process:to_process.clone(),
            to_process,
            remote,
            sender
        }
    }

    pub fn set_from_process(&mut self, from_process:ProcessInfo) {
        self.from_process=from_process;
    }

    //TODO add timeout
    pub fn send_message(&self, message:AnyMessage) -> SendResult {
        let message=Message::new(self.from_process.clone(), MessageContent::Message(message));

        if !self.remote {
            match self.sender.send(message) {
                Ok(_) => SendResult::Ok,
                Err(_) => SendResult::Error( err_msg(self.broken_channel_error()) ),
            }
        }else{
            //TODO
            SendResult::Ok
        }
    }

    //TODO add timeout
    pub fn send_signal(&self, signal:AnySignal) -> SendResult {
        let message=Message::new(self.from_process.clone(), MessageContent::Signal(signal));

        if !self.remote {
            match self.sender.send(message) {
                Ok(_) => SendResult::Ok,
                Err(_) => SendResult::Error( err_msg(self.broken_channel_error()) ),
            }
        }else{
            //TODO
            SendResult::Ok
        }
    }

    pub fn send_message_nonblock(&self, message:AnyMessage) -> SendResult {
        let message=Message::new(self.from_process.clone(), MessageContent::Message(message));

        if !self.remote {
            match self.sender.send(message) {
                Ok(_) => SendResult::Ok,
                Err(_) => SendResult::Error( err_msg(self.broken_channel_error()) ),
            }
        }else{
            //TODO
            SendResult::Ok
        }
    }

    pub fn send_signal_nonblock(&self, signal:AnySignal) -> SendResult {
        let message=Message::new(self.from_process.clone(), MessageContent::Signal(signal));

        if !self.remote {
            match self.sender.send(message) {
                Ok(_) => SendResult::Ok,
                Err(_) => SendResult::Error( err_msg(self.broken_channel_error()) ),
            }
        }else{
            //TODO
            SendResult::Ok
        }
    }

    fn broken_channel_error(&self) -> BrokenChannel {
        BrokenChannel( self.to_process.clone() )
    }

    pub fn get_to_process(&self) -> ProcessInfo {
        self.to_process.clone()
    }
}

/*
pub enum SendResult {
    Signal(ProcessInfo, AnySignal),
    Message(ProcessInfo, AnyMessage),
    Timeout,
    Error(Error)
}
*/