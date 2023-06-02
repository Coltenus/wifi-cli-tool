mod wifi;
mod highlighter;

fn scan_string_with_message(message: &str, echo: bool) -> String {
    use std::io::{stdout,Write};
    use rustyline::Editor;
    use rustyline::{history::FileHistory, DefaultEditor};
    use highlighter::MaskingHighlighter;

    print!("{}", message);
    let _ = stdout().flush();
    let readline: Result<String, rustyline::error::ReadlineError>;
    if !echo {
        let h = MaskingHighlighter::new('*');
        let mut rl = Editor::<MaskingHighlighter, FileHistory>::new();
        rl.as_mut().expect("REASON").set_helper(Some(h));
        readline = rl.as_mut().expect("REASON").readline(": ");
    }
    else {
        let mut rl = DefaultEditor::new().unwrap();
        readline = rl.readline(": ");
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
    use std::io::stdin;
    let mut option: i8 = -1;
    while option != 0 {
        option = -1;
        let line = scan_string_with_message("Enter an option\n1) Get info\n2) Connect\n3) Disconnect\n0) Exit\n", true);
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
                let mut buf = String::new();
                println!("{}", wifi::info::wifi_info());
                stdin().read_line(&mut buf).unwrap();
            },
            2 => {
                let interface = wifi::action::get_interface();
                let name = scan_string_with_message("Enter the name of your wifi\n", true);
                let pass = scan_string_with_message("Enter your password\n", false);
                wifi::action::connect(interface, &name, &pass, true);
            },
            3 => {
                let interface = wifi::action::get_interface();
                wifi::action::disconnect(interface, true);
            },
            _ => {
                if option != -1 && option != 0 {
                    println!("There is no selected option.");
                }
            }
        }
        println!();
    }
}