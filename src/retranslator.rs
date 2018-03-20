
use std::sync::RwLock;

use failure::Error;
use failure::err_msg;

use process::ProcessInfo;

use channel::Sender;
use channel::{AnySignal,AnyMessage};
use channel::SendResult as ChannelSendResult;

use errors::Poisoned;

pub type SendResult=Result<(),Vec<SendProblem>>;

pub struct Retranslator(RwLock<InnerRetranslator>);

pub struct InnerRetranslator {
    subscribers:Vec<Sender>,
}

pub enum SendProblem {
    Timeout(ProcessInfo),
    Error(Error)
}

impl Retranslator {
    pub fn new() -> Self {
        Retranslator( RwLock::new(InnerRetranslator::new()) )
    }

    pub fn subscribe(&self, sender:Sender) -> Result<(),Error> {
        let mut retranslator=match self.0.write() {
            Ok(retranslator) => retranslator,
            Err(_) => return Err( err_msg(self.poisoned_error()) )
        };

        retranslator.subscribe(sender);

        Ok(())
    }

    pub fn unsubscribe(&self, to_process:ProcessInfo) -> Result<bool,Error> {
        let mut retranslator=match self.0.write() {
            Ok(retranslator) => retranslator,
            Err(_) => return Err( err_msg(self.poisoned_error()) )
        };

        retranslator.unsubscribe(to_process)
    }

    pub fn send_message(&self, from_process:ProcessInfo, message:AnyMessage) -> SendResult {
        let retranslator=match self.0.read() {
            Ok(retranslator) => retranslator,
            Err(_) => return Err( vec![SendProblem::Error( err_msg(self.poisoned_error()) )] )
        };

        retranslator.send_message(from_process, message)
    }

    fn poisoned_error(&self) -> Poisoned {
        Poisoned()
    }
}

impl InnerRetranslator {
    pub fn new() -> Self {
        InnerRetranslator {
            subscribers:Vec::with_capacity(4)
        }
    }

    fn subscribe(&mut self, sender:Sender) {
        self.subscribers.push(sender);
    }

    fn unsubscribe(&mut self, to_process:ProcessInfo) -> Result<bool,Error> {
        let mut delete_index=None;

        for (i,subscriber) in self.subscribers.iter().enumerate() {
            if subscriber.get_to_process()==to_process {
                delete_index=Some(i);
                break;
            }
        }

        match delete_index {
            Some(i) => {self.subscribers.remove(i); Ok(true)},
            None => Ok(false)
        }
    }


    //TODO from?
    fn send_message(&self, from_process:ProcessInfo, message:AnyMessage) -> SendResult {
        let mut problems=Vec::new();

        if self.subscribers.len()==1 {
            let subscriber=&self.subscribers[0];
            match subscriber.send_message(message) {
                ChannelSendResult::Ok => {},
                ChannelSendResult::Timeout => problems.push(SendProblem::Timeout(subscriber.get_to_process())),
                ChannelSendResult::Error(error) => problems.push(SendProblem::Error(error))
            }
        }else{
            /*
            for subscriber in self.subscribers.iter() {
                let
                match subscriber.send_message(message) {
                    ChannelSendResult::Ok => {},
                    ChannelSendResult::Timeout => problems.push(SendProblem::Timeout(subscriber.get_to_process())),
                    ChannelSendResult::Error(error) => problems.push(SendProblem::Error(error))
                }
            }
            */
        }

        if problems.len()>0 {
            Ok(())
        }else{
            Err(problems)
        }
    }
}