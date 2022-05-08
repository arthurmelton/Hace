use process_memory::{Architecture, Memory, TryIntoProcessHandle};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
#[cfg(target_os = "windows")]
use std::os::raw::c_void;

#[derive(Clone, Debug)]
pub struct Mem {
    #[cfg(not(target_os = "windows"))]
    handle: (i32, Architecture),
    #[cfg(target_os = "windows")]
    handle: (*mut c_void, Architecture),
}

impl Mem {
    #[must_use]
    pub fn new(x: u32) -> Mem {
        return Mem {
            handle: (x as process_memory::Pid)
                .try_into_process_handle()
                .unwrap(),
        };
    }

    pub fn read<T>(&self, offset: Vec<usize>) -> std::io::Result<&T> {
        let mut item = process_memory::DataMember::<&T>::new(self.handle);
        item.set_offset(offset);
        item.read()
    }

    pub fn write<T>(&self, offset: Vec<usize>, write: &T) -> std::io::Result<()> {
        let mut item = process_memory::DataMember::<&T>::new(self.handle);
        item.set_offset(offset);
        item.write(&write)
    }
}

pub fn pidof(x: &str) -> u32 {
    let mut system = System::new_all();
    system.refresh_all();
    let x = match sysinfo::SystemExt::processes_by_name(&system, x).next() {
        Some(x) => x.pid().as_u32(),
        None => 0,
    };
    x
}
