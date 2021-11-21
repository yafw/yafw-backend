pub mod cpu;

use std::{thread, time};

pub fn statistics_thread_init(){

    // Spawn CPU pooler
    thread::spawn(|| {
        loop {
            // Update data
            cpu::update_statistics();

            // We only want to update cpu stats every second
            thread::sleep(time::Duration::from_millis(1000));
        }
    });

}

