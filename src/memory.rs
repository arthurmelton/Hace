use process_memory::{Architecture, Memory, TryIntoProcessHandle};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

#[derive(Clone, Debug)]
pub struct Mem {
    handle: (i32, Architecture),
}

impl Mem {
    #[must_use]
    pub fn new(x: u32) -> Mem {
        Mem {
            handle: (x as process_memory::Pid)
                .try_into_process_handle()
                .unwrap(),
        }
    }

    pub fn read<T>(&self, offset: Vec<usize>) -> std::io::Result<*mut T> {
        let mut item = process_memory::DataMember::<*mut T>::new(self.handle);
        item.set_offset(offset);
        item.read()
    }

    pub fn write<T>(&self, offset: Vec<usize>, write: &*mut T) -> std::io::Result<()> {
        let mut item = process_memory::DataMember::<*mut T>::new(self.handle);
        item.set_offset(offset);
        item.write(write)
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
