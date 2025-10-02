use std::{ fs, process::exit };

pub fn compile(file_path: &str) {
    if !file_path.ends_with(".nomore") {
        println!("Error: File type not supported");
        exit(1);
    }

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    const INSTRUCTION_SET: [&str; 2] = ["MORE", "NOMORE"];

    let tokens: Vec<String> = contents
        .split_whitespace()
        .map(|t| t.to_uppercase())
        .filter(|t| INSTRUCTION_SET.contains(&t.as_str()))
        .collect();

    let mut idx = 0;

    let mut final_value: Vec<i16> = vec![];

    while idx < tokens.len() {
        if tokens[idx].to_uppercase() != INSTRUCTION_SET[0] {
            idx += 1;
            continue;
        }

        let mut add_count = 0;
        while idx < tokens.len() {
            let token = &tokens[idx];

            if token.to_uppercase() == INSTRUCTION_SET[0] {
                add_count += 1;
            } else if token.to_uppercase() == INSTRUCTION_SET[1] {
                final_value.push(add_count);
                idx += 1;
                break;
            } else {
                println!("Error: Unexpected token '{}' at position {}", token, idx + 1);
                exit(1);
            }

            idx += 1;
        }
    }

    let bytes: Vec<u8> = final_value
        .iter()
        .map(|&x| x as u8)
        .collect();
    let s: &str = std::str::from_utf8(&bytes).expect("Error: Invalid ASCII bytes");

    println!("{}", s);
    exit(0);
}
