
use std::time::{SystemTime};
use std::io::Write;
//use requestty;
//use requestty::Question;

fn prompt(name:&str) -> String {
    let mut line = String::new();
    print!("{}", name);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("Error: Could not read a line");
 
    return line.trim().to_string()
}

pub fn shell() {
    loop {
        let input=prompt("dmud> ");
        match input.as_str() {
            "now" => {
                let unixtime = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
                print!("Current Unix time is {:?}\n", unixtime);
            },
            "exit" => {
                break;
            },
            _ => {
                print!("Unknown command: {}\n", input);
            }
        }
    }
}

