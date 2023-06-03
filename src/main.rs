use std::time::Duration;

mod wifi;
mod highlighter;
mod ping;

fn scan_string_with_message(message: &str, mask: bool) -> String {
    use std::io::{stdout,Write};
    use rustyline::Editor;
    use rustyline::{history::FileHistory, DefaultEditor};
    use highlighter::MaskingHighlighter;

    print!("{}", message);
    let _ = stdout().flush();
    let readline: Result<String, rustyline::error::ReadlineError>;
    if mask {
        let h = MaskingHighlighter::new('*');
        let mut rl = Editor::<MaskingHighlighter, FileHistory>::new();
        rl.as_mut().expect("REASON").set_helper(Some(h));
        readline = rl.as_mut().expect("REASON").readline("> ");
    }
    else {
        let mut rl = DefaultEditor::new().unwrap();
        readline = rl.readline("> ");
    }
    let mut line = String::new();
    match readline {
        Ok(l) => {
            line = l;
        }
        Err(error) => {
            println!("Error: {:?}", error);
        }
    }
    if let Some('\n')=line.chars().next_back() {
        line.pop();
    }
    if let Some('\r')=line.chars().next_back() {
        line.pop();
    }
    line
}

fn main() {
    let mut option: i8 = -1;
    while option != 0 {
        option = -1;
        let line = scan_string_with_message(&("Enter an option\n".to_owned()
        + "1) Get info\n2) Connect\n3) Disconnect\n4) Get ping\n5) Check selected hosts ping\n"
        + "6) Check selected ips\n0) Exit\n: "), false);
        match line.parse::<i8>() {
            Ok(num) => {
                option = num;
            },
            Err(_) => {
                println!("Given string is not a number.");
            }
        }
        match option {
            1 => {
                println!("{}", wifi::info::wifi_info());
                scan_string_with_message("", false);
            }
            2 => {
                let interface = wifi::action::get_interface();
                let name = scan_string_with_message("Enter the name of your wifi\n: ", false);
                let pass = scan_string_with_message("Enter your password\n: ", true);
                wifi::action::connect(interface, &name, &pass, true);
            }
            3 => {
                let interface = wifi::action::get_interface();
                wifi::action::disconnect(interface, true);
            }
            4 => {
                let hosts: Vec<&str> = vec!["google.com", "youtube.com", "reddit.com", "github.com"];
                let mut sum = Duration::ZERO;
                for host in &hosts {
                    let dur = ping::get_ping(host, false);
                    match dur {
                        Ok(duration) => {
                            sum += duration;
                            println!("{} - {:?}", host, duration);
                        }
                        Err(err) => {
                            println!("{}({})", err, host);
                        }
                    }
                }
                if !sum.is_zero() {
                    println!("Avarage - {:?}", sum/hosts.len().try_into().unwrap());
                }
                scan_string_with_message("", false);
            }
            5 => {
                let hosts_str = scan_string_with_message("Enter host\n", false);
                let hosts = hosts_str.split(' ');
                let mut sum = Duration::ZERO;
                let mut count = 0;
                for host in hosts {
                    let dur = ping::get_ping(host, false);
                    match dur {
                        Ok(duration) => {
                            sum += duration;
                            count += 1;
                            println!("{} - {:?}", host, duration);
                        }
                        Err(err) => {
                            println!("{}({})", err, host);
                        }
                    }
                }
                if !sum.is_zero() {
                    println!("Avarage - {:?}", sum/count);
                }
                scan_string_with_message("", false);
            }
            6 => {
                let ips_str = scan_string_with_message("Enter host\n", false);
                let ips = ips_str.split(' ');
                let mut sum = Duration::ZERO;
                let mut count = 0;
                for ip in ips {
                    let dur = ping::get_ping(ip, true);
                    match dur {
                        Ok(duration) => {
                            sum += duration;
                            count += 1;
                            println!("{} - {:?}", ip, duration);
                        }
                        Err(err) => {
                            println!("{}({})", err, ip);
                        }
                    }
                }
                if !sum.is_zero() {
                    println!("Avarage - {:?}", sum/count);
                }
                scan_string_with_message("", false);
            }
            _ => {
                if option != -1 && option != 0 {
                    println!("There is no selected option.");
                }
            }
        }
        println!();
    }
}