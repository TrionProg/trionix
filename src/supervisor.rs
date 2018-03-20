
use std::collections::HashMap;
use std::collections::hash_map::Entry;

use channel::{Sender,SendResult};
use process::{ProcessInfo, ProcessClassCode};

use messages::MsgSender;

/*

pub trait SupervisorTrait {

}

struct Process {
    join_handle:JoinHandle<()>,
    sender:Sender
}

pub struct MainSupervisor(Arc<InnerSupervisor>);

pub struct InnerSupervisor {
    processes:Vec<(ProcessInfo, Sender)>
}

impl InnerSupervisor {
    pub fn new() -> Self {
        InnerSupervisor {
            processes:Vec::new()
        }
    }

    pub fn kill(&mut self) {
        for &(ref process_info,ref sender) in self.processes.iter() {

        }
    }
}

pub struct WaitingForSender {
    waiting_for_sender:HashMap<ProcessClassCode,Vec<Sender>>
}

impl WaitingForSender {
    pub fn new() -> Self {
        WaitingForSender {
            waiting_for_sender:HashMap::new();
        }
    }

    pub fn add(&mut self, process_class_code:ProcessClassCode, sender:Sender) {
        match self.waiting_for_sender.entry(process_class_code) {
            Entry::Occupied(ref mut e) => e.get_mut().push(sender),
            Entry::Vacant(ref mut e) => {e.insert(vec![sender]);}
        }
    }
    
    pub fn send_all(&mut self, process_class_code:ProcessClassCode, sender:&Sender) {
        match self.waiting_for_sender.get(&process_class_code) {
            Some(ref mut group) => {
                let mut problems=Vec::new();

                for waiting_sender in group.iter() {
                    let message=Box::new(MsgSender(sender.clone()));
                    match waiting_sender.send_message(message) {
                        SendResult::Ok => {},
                        SendResult::Timeout => problems.push(SendProblem::Timeout(waiting_sender.get_to_process())),
                        SendResult::Error(error) => problems.push(SendProblem::Error(error))
                    }
                }
            }
        }
    }
}
*/