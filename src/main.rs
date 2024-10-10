use std::path::Path;
pub mod functions;

fn main() {
    use std::io::stdin;
    println!("Hello, world!");
    let mut console_buffer = String::new();
    stdin().read_line(&mut console_buffer).expect("Did not enter a correct string");

    let func_to_call = console_buffer.trim();
    if func_to_call == "echo"{
        functions::echo();
    }
    else if func_to_call == "ls"{
        functions::ls()
    }
    else if func_to_call == "cat"{
        functions::cat()
    }
}
