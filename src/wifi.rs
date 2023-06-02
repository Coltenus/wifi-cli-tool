extern crate wifi_rs;

pub mod info {
    use std::process::Command;

    #[cfg(target_os = "windows")]
    pub fn wifi_info() -> String {
        let output = Command::new("netsh")
        .args(&["wlan", "show", "interfaces"])
        .output()
        .expect("Failed to execute netsh");

        String::from_utf8_lossy(&output.stdout).to_string()
    }

    #[cfg(target_os = "linux")]
    pub fn wifi_info() -> String {
        let output = Command::new("iwconfig")
        .output()
        .expect("Failed to execute iwconfig");

        String::from_utf8_lossy(&output.stdout)
    }
}

pub mod action {
    use wifi_rs::prelude::*;
    use wifi_rs::WiFi;

    pub fn get_interface() -> wifi_rs::WiFi {
        let config = Some(Config {
            interface: Some("wlo1"),
        });
    
        WiFi::new(config)
    }
    use core::time;
    use std::thread;

    use super::info::wifi_info;
    pub fn connect(interface: WiFi, name: &str, pass: &str, need_print: bool) -> WiFi {
        let mut interf = interface;
        interf = disconnect(interf, false);
        match interf.connect(name, pass) {
            Ok(_) => {
                if need_print {
                    let mut counter: i8 = 0;
                    while !wifi_info().contains(" connected") {
                        if counter > 15 {
                            break;
                        }
                        thread::sleep(time::Duration::from_millis(200));
                        counter += 1;
                    }
                    if counter <= 15 {
                        println!("Connection was done successfully.");
                    }
                    else {
                        println!("Can't connect.");
                    }
                }
                else {
                    thread::sleep(time::Duration::from_millis(800));
                }
            },
            Err(_) => {
                if need_print {
                    println!("There was an error while connection.");
                }
            },
        }
        interf
    }

    pub fn disconnect(interface: WiFi, need_print: bool) -> WiFi {
        let interf = interface;

        match interf.disconnect() {
            Ok(_) => {
                if need_print {
                    println!("Wifi was disconnected.");
                }
                thread::sleep(time::Duration::from_millis(800));
            },
            Err(_) => {
                if need_print {
                    println!("Something went wrong.");
                }
            }
        }

        interf
    }
}