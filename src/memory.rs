use process_memory::{Memory, TryIntoProcessHandle};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

static mut SAVED_PID: u32 = 0;

pub fn pidof(x: &str) -> u32 {
    let mut system = System::new_all();
    system.refresh_all();
    let x = match sysinfo::SystemExt::processes_by_name(&system, x).next() {
        Some(x) => x.pid().as_u32(),
        None => 0,
    };
    x
}

pub fn set_pid(x: u32) {
    unsafe {
        SAVED_PID = x;
    }
}

/// Prob a better way of doing this but I dont know how to ///

pub fn read_bool(offset: Vec<usize>) -> bool {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<bool>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_bool(offset: Vec<usize>, write: bool) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<bool>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_i8(offset: Vec<usize>) -> i8 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i8>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_i8(offset: Vec<usize>, write: i8) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i8>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_i16(offset: Vec<usize>) -> i16 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i16>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_i16(offset: Vec<usize>, write: i16) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i16>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_i32(offset: Vec<usize>) -> i32 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i32>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_i32(offset: Vec<usize>, write: i32) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i32>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_i64(offset: Vec<usize>) -> i64 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i64>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_i64(offset: Vec<usize>, write: i64) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i64>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_i128(offset: Vec<usize>) -> i128 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i128>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_i128(offset: Vec<usize>, write: i128) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<i128>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_isize(offset: Vec<usize>) -> isize {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<isize>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_isize(offset: Vec<usize>, write: isize) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<isize>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_u8(offset: Vec<usize>) -> u8 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u8>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_u8(offset: Vec<usize>, write: u8) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u8>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_u16(offset: Vec<usize>) -> u16 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u16>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_u16(offset: Vec<usize>, write: u16) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u16>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_u32(offset: Vec<usize>) -> u32 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u32>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_u32(offset: Vec<usize>, write: u32) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u32>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_u64(offset: Vec<usize>) -> u64 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u64>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_u64(offset: Vec<usize>, write: u64) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u64>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_u128(offset: Vec<usize>) -> u128 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u128>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_u128(offset: Vec<usize>, write: u128) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<u128>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_usize(offset: Vec<usize>) -> usize {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<usize>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_usize(offset: Vec<usize>, write: usize) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<usize>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_string(offset: Vec<usize>) -> String {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<&str>::new(handle);
        item.set_offset(offset);
        item.read().unwrap().to_string()
    }
}

pub fn write_string(offset: Vec<usize>, write: String) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<&str>::new(handle);
        item.set_offset(offset);
        item.write(&write.as_str()).unwrap();
    }
}
pub fn read_f32(offset: Vec<usize>) -> f32 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<f32>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_f32(offset: Vec<usize>, write: f32) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<f32>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_f64(offset: Vec<usize>) -> f64 {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<f64>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_f64(offset: Vec<usize>, write: f64) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<f64>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
pub fn read_char(offset: Vec<usize>) -> char {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<char>::new(handle);
        item.set_offset(offset);
        item.read().unwrap()
    }
}

pub fn write_char(offset: Vec<usize>, write: char) {
    unsafe {
        let handle = (SAVED_PID as process_memory::Pid)
            .try_into_process_handle()
            .unwrap();
        let mut item = process_memory::DataMember::<char>::new(handle);
        item.set_offset(offset);
        item.write(&write).unwrap();
    }
}
