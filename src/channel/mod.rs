
pub mod sender;
pub use self::sender::{Sender,SendResult};

pub mod receiver;
pub use self::receiver::{Receiver,RecvResult,NonBlockRecvResult};

pub mod message;
pub use self::message::{SignalTrait, MessageTrait};
pub use self::message::{SignalCode, MessageCode, AnySignal, AnyMessage};
pub use self::message::{Message, MessageContent};

pub mod signals;

use std::sync::mpsc;
use process::ProcessInfo;

pub fn create_channel(to_process_info: ProcessInfo) -> (Sender, Receiver) {
    let (sender, receiver) = mpsc::channel();
    (Sender::new(to_process_info.clone(), sender), Receiver::new(to_process_info.clone(), receiver))
}