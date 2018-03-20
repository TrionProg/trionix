
use std::collections::VecDeque;
use std::sync::mpsc;
use std::sync::mpsc::TryRecvError;

use failure::Error;
use failure::err_msg;

use process::{ProcessClassTrait, ProcessInfo, ProcessID, LocalProcessID};

use errors::BrokenChannel;

use super::{Message, MessageContent};
use super::MessageTrait;
use super::{AnySignal,AnyMessage};

pub struct Receiver {
    process_info:ProcessInfo,
    receiver:mpsc::Receiver< Message >,
    deferred_messages:VecDeque< (ProcessInfo,AnyMessage) >
}

pub enum RecvResult {
    Signal(ProcessInfo, AnySignal),
    Message(ProcessInfo, AnyMessage),
    Timeout,
    Error(Error)
}

pub enum NonBlockRecvResult {
    Signal(ProcessInfo, AnySignal),
    Message(ProcessInfo, AnyMessage),
    None,
    Error(Error)
}

impl Receiver {
    pub fn new(process_info:ProcessInfo, receiver:mpsc::Receiver< Message >) -> Self {
        Receiver{
            process_info,
            receiver,
            deferred_messages:VecDeque::with_capacity(4)
        }
    }

    //TODO timeout
    pub fn recv<F>(&mut self, filter:F) -> RecvResult where
        F:Fn(&ProcessInfo,&AnyMessage) -> bool
    {
        //Gibt es ein Signal,das hat mehr Priorität als Command?
        loop {
            match self.receiver.try_recv() {
                Ok(message) => {
                    match message.content {
                        MessageContent::Signal(signal) => return RecvResult::Signal(message.from_process, signal),
                        MessageContent::Message(msg) => self.deferred_messages.push_front( (message.from_process, msg) )
                    }
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return RecvResult::Error( err_msg(self.broken_channel_error()) )
            }
        }

        //Gibt es das Message in deferred_messages?
        let mut delete=None;
        for (i,&(ref from_process, ref message)) in self.deferred_messages.iter().enumerate() {
            if filter(from_process,message) {
                delete=Some(i);
                break;
            }
        }

        match delete {
            Some(index) => {
                let (from_process,message)=self.deferred_messages.remove(index).unwrap();
                return RecvResult::Message(from_process, message);
            },
            None => {}
        }

        //Warte
        loop {
            match self.receiver.recv() {
                Ok(message) => {
                    match message.content {
                        MessageContent::Signal(signal) => return RecvResult::Signal(message.from_process, signal),
                        MessageContent::Message(msg) => self.deferred_messages.push_front( (message.from_process, msg) )
                    }
                },
                Err(_) => return RecvResult::Error( err_msg(self.broken_channel_error()) )
            }
            match self.receiver.recv() {
                Ok(message) => {
                    match message.content {
                        MessageContent::Signal(signal) => return RecvResult::Signal(message.from_process, signal),
                        MessageContent::Message(msg) => {
                            if filter(&message.from_process, &msg) {
                                return RecvResult::Message(message.from_process, msg);
                            }else{
                                self.deferred_messages.push_front( (message.from_process, msg) );
                            }
                        }
                    }
                },
                Err(_) => return RecvResult::Error( err_msg(self.broken_channel_error()) ),
            }
        }
    }

    pub fn recv_nonblock<F>(&mut self, filter:F) -> NonBlockRecvResult where
        F:Fn(&ProcessInfo,&AnyMessage) -> bool
    {
        //Gibt es ein Signal,das hat mehr Priorität als Command?
        loop {
            match self.receiver.try_recv() {
                Ok(message) => {
                    match message.content {
                        MessageContent::Signal(signal) => return NonBlockRecvResult::Signal(message.from_process, signal),
                        MessageContent::Message(msg) => self.deferred_messages.push_front( (message.from_process, msg) )
                    }
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return NonBlockRecvResult::Error( err_msg(self.broken_channel_error()) )
            }
        }

        //Gibt es das Message in deferred_messages?
        let mut delete=None;
        for (i,&(ref from_process, ref message)) in self.deferred_messages.iter().enumerate() {
            if filter(from_process,message) {
                delete=Some(i);
                break;
            }
        }

        match delete {
            Some(index) => {
                let (from_process,message)=self.deferred_messages.remove(index).unwrap();
                NonBlockRecvResult::Message(from_process, message)
            },
            None => NonBlockRecvResult::None
        }
    }

    fn broken_channel_error(&self) -> BrokenChannel {
        BrokenChannel( self.process_info.clone() )
    }
}