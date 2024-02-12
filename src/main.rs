use std::env;
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::io::Error;
use clipboard::*;

const SEPARATOR : &str = "=";

fn main() {
    init();
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        println!("Usage \nPassword_helper <identifier>\t\t - to get password for identifier");
        println!(
            "Password_helper <identifier> <password>\t - to set/update password for identifier"
        );
        println!("Password_helper -d <identifier>\t\t - to delete identifier");
        return;
    } else if args.len() >= 2 {
        if args[1] == "-d" && args.len() == 3 {
            let identifier = &args[2];
            delete_password(identifier);
            return;
        } else if args.len() == 3 {
            let identifier = &args[1];
            let password = &args[2];
            set_password(identifier, password);
            return;
        } else if args.len() == 2 {
            let identifier = &args[1];
            let password = match get_password(identifier) {
                Ok(p) => p,
                Err(e) => {
                    println!("{}", e);
                    return;
                }
            };
            copy_to_clipboard(&password);
            println!("Password for {} is {}", identifier, password);
        } else {
            println!("Usage password_helper <identifier>\t\t - to get password for identifier");
            println!(
                "Usage password_helper <identifier> <password>\t - to set password for identifier"
            );
        }
    }
}

fn copy_to_clipboard(data: &str) {
    match ClipboardContext::new() {   
        Ok(mut c) => c.set_contents(data.to_owned()).unwrap(),
        Err(e) => {
            println!("Error creating clipboard context: {}", e);
            return;
        }
    };
}

fn init() {
    let _ = match fs::OpenOptions::new().create(true).write(true).open("passwords.txt") {
        Ok(_) => (),
        Err(e) => println!("Error creating file: {}", e),
    };
}

fn set_password(identifier: &str, password: &str) {
    match get_password(identifier) {
        Ok(_) => {
            update_password(identifier, password);
            return;
        }
        Err(_) => (),
    }
    let mut file = fs::OpenOptions::new().append(true).open("passwords.txt").unwrap();
    println!("Password for {} set to {}", identifier, password);
    match writeln!(file, "{}{}{}", identifier,SEPARATOR, password) {
        Ok(_) => (),
        Err(e) => println!("Error writing to file: {}", e),
    }
}

fn update_password(identifier: &str, password: &str) {
    let contents = match fs::read_to_string("passwords.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };
    let mut new_contents = String::new();
    for line in contents.lines() {
        let mut parts = line.split(SEPARATOR);
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        if key == identifier {
            if password.len() == 0 {
                continue;
            } else {
                new_contents.push_str(&format!("{}{}{}\n", identifier, SEPARATOR, password));
            }
        } else {
            new_contents.push_str(&format!("{}{}{}\n", key, SEPARATOR, value));
        }
    }
    match fs::write("passwords.txt", new_contents.as_bytes()) {
        Ok(_) => (),
        Err(e) => println!("Error writing to file: {}", e),
    }
    println!("Password for {} updated to {}", identifier, password);
}

fn delete_password(identifier: &str) {
    let contents = match fs::read_to_string("passwords.txt") {
        Ok(f) => f,
        Err(e) => {
            println!("Error reading file: {}", e);
            return;
        }
    };
    let mut new_contents = String::new();
    for line in contents.lines() {
        let mut parts = line.split(SEPARATOR);
        if parts.clone().count() == 1 {
            continue;
        } 
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        if key == identifier {
            continue;
        } else {
            new_contents.push_str(&format!("{}{}{}\n", key, SEPARATOR, value));
        }
    }
    
    match fs::write("passwords.txt", new_contents.as_bytes()) {
        Ok(_) => (),
        Err(e) => println!("Error writing to file: {}", e),
    }
    println!("Password for {} deleted", identifier);
}

fn get_password(identifier: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = match fs::read_to_string("passwords.txt") {
        Ok(f) => f,
        Err(e) => {
            return Err(Box::new(e));
        }
    };
    let mut keys = Vec::new();
    for line in contents.lines() {
        let mut parts = line.split(SEPARATOR);
        let key = parts.next().unwrap();
        let value = parts.next().unwrap();
        if key == identifier {
            return Ok(value.to_string());
        } else if key.contains(identifier) {
            keys.push(key);
        }
    }
    if keys.len() > 0 {
        println!("Did you mean:");
        let mut index = 1;
        for key in keys {
            println!("{}) {}", index, key);
            index += 1;
        }
    }
    Err(Box::new(Error::new(ErrorKind::NotFound, "No password found")))
}
