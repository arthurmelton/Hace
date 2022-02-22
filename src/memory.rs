use sysinfo::{System, SystemExt, ProcessExt, PidExt};
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
