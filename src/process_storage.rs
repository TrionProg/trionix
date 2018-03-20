use object_pool::growable::{Pool,ID};

use std::sync::{Mutex,RwLock};
use std::thread::Builder as ThreadBuilder;
use std::thread::JoinHandle;

use failure::Error;
use failure::err_msg;

use process::{ProcessInfo, ProcessID, LocalProcessID, ProcessClassCode, ProcessEnvironmentTrait};
use channel::{Sender, Receiver, create_channel};

use errors::Poisoned;

static mut PROCESS_STORAGE: *const ProcessStorage = 0 as *const ProcessStorage;

pub struct ProcessStorage {
    local_processes:Mutex< Pool<ProcessHandle,ProcessHandle> >,
    process_classes:RwLock< Pool<String,String> >
}

struct ProcessHandle {
    process_class_code:ProcessClassCode,
    join_handle:JoinHandle<()>
}

impl ProcessStorage {
    pub fn new() -> Self {
        ProcessStorage {
            local_processes:Mutex::new(Pool::new()),
            process_classes:RwLock::new(Pool::new())
        }
    }

    pub fn add_process_class(&self, process_class_name:String) -> Result<ProcessClassCode,Error> {
        let mut process_classes=match self.process_classes.write() {
            Ok(process_classes) => process_classes,
            Err(_) => return Err( err_msg(self.process_classes_poisoned_error()) )
        };

        let id=process_classes.insert(process_class_name);
        let process_class_code=id.slot_index as ProcessClassCode;

        Ok(process_class_code)
    }

    //TODO do not clone Class Name
    pub fn get_process_class_name(&self, process_class_code:ProcessClassCode) -> Result<Option<String>,Error> {
        let mut process_classes=match self.process_classes.read() {
            Ok(process_classes) => process_classes,
            Err(_) => return Err( err_msg(self.process_classes_poisoned_error()) )
        };

        let process_class_name=process_classes.get(ID::new(process_class_code as usize));

        match process_class_name {
            Some(process_class_name) => Ok(Some(process_class_name.clone())),
            None => Ok(None)
        }
    }

    pub fn create_process<E:ProcessEnvironmentTrait>(&self,
        process_class_code:ProcessClassCode,
        process_function:fn(ProcessInfo,Receiver,E
    ) -> Result<(),Error>, environment:E) -> Result<Sender,Error> {
        let process_class_name = match self.get_process_class_name(process_class_code)? {
            Some(process_class_name) => process_class_name,
            None => unreachable!(),
        };

        let mut local_processes=match self.local_processes.lock() {
            Ok(local_processes) => local_processes,
            Err(_) => return Err( err_msg(self.local_processes_poisoned_error()) )
        };

        let id=local_processes.future_id();
        let process_id=ProcessID::new_local(id.slot_index as LocalProcessID);
        let process_info=ProcessInfo::new(process_class_code, process_id);

        let (sender,receiver)=create_channel(process_info.clone());

        let join_handle=match ThreadBuilder::new().name(process_class_name).spawn(move|| {
            //TODO recv supervisor match
            match process_function(process_info, receiver, environment) {//TODO:supervisor
                Ok(_) => {},
                Err(e) => {},//TODO send to supervisor
            }
        }) {
            Ok(join_handle) => join_handle,
            Err(e) => unreachable!()
        };

        let process_handle=ProcessHandle::new(process_class_code, join_handle);

        assert_eq!(local_processes.insert(process_handle), id);

        Ok(sender)
    }

    pub fn delete_process(&self, process_info:ProcessInfo) -> Result<(),Error> {
        let process_handle={
            let mut local_processes=match self.local_processes.lock() {
                Ok(local_processes) => local_processes,
                Err(_) => return Err( err_msg(self.local_processes_poisoned_error()) )
            };

            let id=ID::new(process_info.process_id.local_process_id as usize);
            /*
            let process_handle=match local_processes.get(&id) {
                Some(process_handle) => process_handle.clone(),
                None => unreachable!()
            };
            */

            local_processes.remove(id);

            //process_handle
        };

        //process_handle.join_handle.join();//TODO panic result.. how and when to send it to supervisor? After erlang

        Ok(())
    }

    fn local_processes_poisoned_error(&self) -> Poisoned {
        Poisoned()
    }

    fn process_classes_poisoned_error(&self) -> Poisoned {
        Poisoned()
    }
}

impl ProcessHandle {
    pub fn new(process_class_code:ProcessClassCode, join_handle:JoinHandle<()>) -> Self {
        ProcessHandle {
            process_class_code,
            join_handle
        }
    }
}

pub fn create_process_storage() {
    let process_storage=Box::new(ProcessStorage::new());
    unsafe{PROCESS_STORAGE=Box::into_raw(process_storage);}
}

pub fn get_process_storage() -> &'static ProcessStorage {
    unsafe{&*(PROCESS_STORAGE)}
}

pub fn delete_process_storage() {
    unsafe{
        if PROCESS_STORAGE!=0 as *const ProcessStorage {
            let local_storage=Box::from_raw(PROCESS_STORAGE as *mut ProcessStorage);
            PROCESS_STORAGE = 0 as *const ProcessStorage;

            //TODO drop ProcessStorage
        }
    }
}