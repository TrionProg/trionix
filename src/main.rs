extern crate trionix;

#[macro_use]
extern crate failure;
extern crate failure_derive;

use trionix::channel::create_channel;

use std::thread;
use trionix::channel::{Receiver,RecvResult};
use trionix::process::{ProcessInfo,ProcessID};

use trionix::node::set_node_id;
use trionix::process_storage::{create_process_storage,delete_process_storage,get_process_storage};
use trionix::process::ProcessEnvironmentTrait;

use failure::Error;

/*

use trionix::channel::RecvResult;

fn foo2() -> RecvResult {
    RecvResult::Timeout
}

fn foo() -> Result<(),()> {
    foo2()?;

    Ok(())
}
*/

struct ProcAEnv{
    x:i32,
    y:f32
}

impl ProcessEnvironmentTrait for ProcAEnv{}

fn proc_a(proc_info:ProcessInfo, receiver:Receiver, env:ProcAEnv) -> Result<(),Error> {
    println!("{} {}",proc_info,env.y);

    Ok(())
}

fn main() {
    set_node_id(2);
    create_process_storage();

    let process_class_code=get_process_storage().add_process_class("LOL".to_string()).unwrap();

    let env=ProcAEnv{
        x:43,
        y:89.2
    };

    let sender=get_process_storage().create_process(process_class_code, proc_a, env).unwrap();

    let env=ProcAEnv{
        x:48,
        y:829.2
    };

    let sender=get_process_storage().create_process(process_class_code, proc_a, env).unwrap();

    thread::sleep_ms(1000)
    /*

    let (sender,mut receiver)=create_channel(ProcessInfo::new(1,ProcessID::new_local(3)));

    let th=thread::spawn(move || {
        match receiver.recv(|from,msg|{
            true
        }){
            RecvResult::Message(from,msg) => {},
            _ => panic!("aaa")
        }
    });

    th.join();

    println!("good");

    delete_process_storage();
    */

    //foo();
}