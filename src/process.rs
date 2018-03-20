use std::fmt;

use node::NodeID;
use node::get_node_id;

pub type ProcessClassCode=u32;//TODO u16?
pub type LocalProcessID=u32;

pub trait ProcessClassTrait {
    fn code() -> ProcessClassCode;
    fn name() -> &'static str;
}

pub trait ProcessEnvironmentTrait:Send+'static {

}

#[derive(Eq,PartialEq,Clone)]
pub struct ProcessID {
    pub node_id:NodeID,
    pub local_process_id:LocalProcessID,
}

impl ProcessID {
    pub fn new_local(local_process_id:LocalProcessID) -> Self {
        ProcessID {
            node_id:get_node_id(),
            local_process_id
        }
    }

    pub fn new_remote(node_id:NodeID, local_process_id:LocalProcessID) -> Self {
        ProcessID {
            node_id,
            local_process_id
        }
    }

    pub fn is_local(&self) -> bool {
        self.node_id==get_node_id()
    }
}

impl fmt::Debug for ProcessID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO get type of node??
        write!(f, "Process ID{{ node:#{}, id:#{} }}", self.node_id, self.local_process_id)
    }
}

impl fmt::Display for ProcessID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO get type of node
        write!(f, "Process ID{{ node:#{}, id:#{} }}", self.node_id, self.local_process_id)
    }
}

#[derive(Eq,PartialEq,Clone)]
pub struct ProcessInfo {
    pub process_class_code:ProcessClassCode,
    pub process_id:ProcessID
}

impl ProcessInfo {
    pub fn new(process_class_code:ProcessClassCode, process_id:ProcessID) -> Self {
        ProcessInfo{
            process_class_code,
            process_id
        }
    }

    pub fn is_local(&self) -> bool {
        self.process_id.is_local()
    }
}

impl fmt::Debug for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO get types of node and process ??
        write!(f, "Process{{ type:#{}, node:#{}, id:#{} }}", self.process_class_code, self.process_id.node_id, self.process_id.local_process_id)
    }
}

impl fmt::Display for ProcessInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //TODO get types of node and process
        write!(f, "Process{{ type:#{}, node:#{}, id:#{} }}", self.process_class_code, self.process_id.node_id, self.process_id.local_process_id)
    }
}