mod logic;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let command = &args[1];

    const COMMAND_LIST: [&str; 2] = ["run", "help"];

    if command.contains(COMMAND_LIST[0]) {
        if args.len() == 2 {
            logic::help::run_help();
        }

        let file_path = &args[2];

        logic::compile::compile(file_path);
    } else {
        logic::help::help();
    }
}
