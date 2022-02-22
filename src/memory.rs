use sysinfo::{System, SystemExt, ProcessExt, PidExt};
use process_memory::{TryIntoProcessHandle, Memory};

pub fn pidof(x:&str) -> u32 {
    let mut system = System::new_all();
    system.refresh_all();
    let x = match sysinfo::SystemExt::processes_by_name(&system,x).nth(0) {
        Some(x) => {
            x.pid().as_u32()
        },
        None => {
            0
        }
    };
    return x;
}

pub fn read_bool(pid:u32, offset:Vec<usize>) -> bool {
    let handle = (pid as process_memory::Pid).try_into_process_handle().unwrap();
    let mut item = process_memory::DataMember::<bool>::new(handle);
    item.set_offset(offset);
    return item.read().unwrap();
}

pub fn write_bool(pid:u32, offset:Vec<usize>, write:bool) {
    let handle = (pid as process_memory::Pid).try_into_process_handle().unwrap();
    let mut item = process_memory::DataMember::<bool>::new(handle);
    item.set_offset(offset);
    return item.write(&write).unwrap();
}
