use std::io::{self, BufRead};
mod tree;
mod test_tree;

use tree::Tree;

#[derive(Debug)]
enum Command {
    Insert{age: i32, name: String},
    Erase{age: i32, name: String},
    Contains{age: i32, name: String},
    Print,
    Reset,
    Exit,
    Error(String)
}

fn parse_command(input: String) -> Command {
    let command_items: Vec<&str> = input.split_whitespace().collect();
    if command_items.len() == 0 {
        Command::Error("invalid command (empty line)".to_string())
    } else {
        match (command_items[0], command_items.len()) {
            ("p", 1) => Command::Print,
            ("q", 1) => Command::Exit,
            ("x", 1) => Command::Reset,
            ("i", 3) => {
                if let Ok(age) = command_items[1].parse::<i32>() {
                    Command::Insert{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },
            ("e", 3) => {
                if let Ok(age) = command_items[1].parse::<i32>() {
                    Command::Erase{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },
            ("c", 3) => {
                if let Ok(age) = command_items[1].parse::<i32>() {
                    Command::Contains{age: age, name: command_items[2].to_string()}
                } else {
                    Command::Error("unable to parse int (age).".to_string())
                }
            },

            (_, _) => Command::Error("invalid command.".to_string())
        }
    }
}

pub fn command_loop(br: &mut dyn BufRead) {

    let mut tree = Tree::new();

    loop {
        let mut input = String::new();
        
        match br.read_line(&mut input) {
            Ok(0) => {
                // End of file
                break;
            }
            Ok(_) => {
                match parse_command(input) {
                    Command::Insert{age, name} => {
                        tree.insert(age, name);
                    },
                    Command::Erase{age, name} => {
                        tree.erase(age, name);
                    },
                    Command::Contains{age, name} => {
                        match tree.find(age,name) {
                            true => {println!("y")}
                            false => {println!("n")}
                        }
                    }
                    Command::Print => {
                        tree.print();
                    },
                    Command::Reset => {
                        tree.delete();
                        tree = Tree::new();
                    },
                    Command::Exit => {
                        break;
                    },
                    Command::Error(error) => {
                        eprintln!("Error: {}", error);
                    }
                }
            }
            Err(error) => eprintln!("Error: {}", error),
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    command_loop(&mut handle);
}