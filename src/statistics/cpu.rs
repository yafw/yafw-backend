use std::fs;

pub fn update_statistics() {
    let proc_cpu = fs::read_to_string("/proc/stat").expect("Error reading /proc/cpu");

    
}
