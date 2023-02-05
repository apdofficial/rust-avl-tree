#[macro_use]
extern crate afl;

#[path = "../../src/main.rs"] mod main;

fn main() {
    fuzz!(|data: &[u8]| {
        let mut mut_data = data.clone();
        main::command_loop(&mut mut_data);
    });
}

