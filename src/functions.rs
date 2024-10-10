pub fn echo(){
    use std::io::stdin;
    let mut consoleBuf = String::new();
    stdin().read_line(&mut consoleBuf).expect("msg");
    println!("{consoleBuf}");
}

use core::str;
use std::ffi::OsString;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::fs::File;
use std::io::stdin;

pub fn ls() {
    let mut console_buffer = String::new();
    stdin().read_line(&mut console_buffer).expect("Did not enter a correct string");
    let path_str_arg = console_buffer.trim();
    let path_arg = Path::new(&path_str_arg);

    if path_arg.is_dir() {
        for entry in fs::read_dir(path_arg).unwrap() {
            let unwrapped_entry = entry.unwrap();
            let mut file_name = unwrapped_entry
            .file_name()
            .into_string().unwrap();
            if unwrapped_entry.file_type().unwrap().is_dir(){
                file_name = "Folder | ".to_owned() + file_name.as_str();
            }
            else if unwrapped_entry.file_type().unwrap().is_file(){
                file_name = "File | ".to_owned() + file_name.as_str();
            }
            println!("{}", file_name);
        }
    }
    else {
        println!("NOT A DIRECTORY");
    }
}

pub fn cat(){

    let mut console_buffer = String::new();
    stdin().read_line(&mut console_buffer).expect("Did not enter a correct string");
    let path_str_arg = console_buffer.trim();
    let first_file = Path::new(&path_str_arg);

    let mut console_buffer_second = String::new();
    stdin().read_line(&mut console_buffer_second).expect("Did not enter a correct string");
    let path_str_arg_second = console_buffer_second.trim();
    let second_file = Path::new(&path_str_arg_second);

    if first_file.is_file()
    {
        if second_file.is_file()
        {
            let contents_first = std::fs::read(first_file).unwrap();
            let contents_second = std::fs::read(second_file).unwrap();
            let contents_first_str = str::from_utf8(contents_first.as_ref()).unwrap();

            let contents_second_str = str::from_utf8(contents_second.as_ref());
            let contents_final = contents_first_str.to_owned() + contents_second_str.unwrap();

            let mut console_buffer_final = String::new();
            stdin().read_line(&mut console_buffer_final).expect("Did not enter a correct string");
            let path_str_final = console_buffer_final.trim();
            let path_final = Path::new(path_str_final);

            let mut file = File::create(path_final).unwrap();
            file.write(contents_final.as_bytes());
        }
        else {
            println!("NOT A FILE {}", console_buffer);
            return;
        }
    }
    else {
        println!("NOT A FILE {}", console_buffer_second);
    }
}
