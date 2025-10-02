use std::process::exit;

pub fn help() {
    println!(
        "\
NOMORE HELP

Usage:
  NOMORE run <filename>
  NOMORE help

Description:
  A command-line tool that runs my custom esoteric language (Brainf*ck-inspired).

Commands:
  run <filename>           Run the specified .nomore file.
  help                     Show this help message.

Examples:
  NOMORE run pls.nomore
  NOMORE help

For more info, visit: https://github.com/haidarbahzi/NOMORE-Interpreter
"
    );
    exit(0);
}

pub fn run_help() {
    println!(
        "\
Error: Missing required argument '<filename>'

Usage:
  NOMORE run <filename>

Try 'NOMORE help' for more information."
    );
    exit(1);
}
