use sysinfo::{NetworkExt, System, SystemExt};
use sysinfo::NetworksExt;

pub fn get_bytes(system: &mut System, last_state: &mut u64, mode: u8) -> u64 {
    system.refresh_networks();

    if let Some((_name, network)) = system.networks().iter().find(|&(name, _net)| name == "Wi-Fi") {
        let buffer = *last_state;
        match mode {
            1 => {
                *last_state = network.total_received();
            }
            2 => {
                *last_state = network.total_transmitted();
            }
            _ => {}
        }
        if buffer == 0 {
            return 0;
        }
        return *last_state - buffer;
    }

    0
}

pub fn get_system() -> System {
    return System::new_all();
}

pub fn refresh_system(system: &mut System) {
    system.refresh_all();
}