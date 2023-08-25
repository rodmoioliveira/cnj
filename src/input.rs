use std::io::prelude::*;

pub trait HasPositionalInputs {
    fn get_positional_input(&self) -> Vec<String>;
}

pub fn is_positional(input: &[String]) -> bool {
    input[0] != "-"
}

pub fn is_stdin(input: &[String]) -> bool {
    !is_positional(input)
}

pub fn is_valid(input: &[String]) -> bool {
    (is_stdin(input) && grep_cli::is_readable_stdin()) || (!is_stdin(input) && is_positional(input))
}

pub fn get(args: impl HasPositionalInputs) -> Vec<String> {
    let positional_input = args.get_positional_input();
    let stdin_input = if is_stdin(&positional_input) && grep_cli::is_readable_stdin() {
        let mut stdin_input: Vec<String> = Vec::new();

        let stdin = std::io::stdin();
        for line in stdin.lock().lines() {
            let unquoted_input = line.unwrap().replace('"', "").to_owned();
            stdin_input.push(unquoted_input);
        }

        Some(stdin_input)
    } else {
        None
    };

    if let Some(input) = stdin_input {
        input
    } else {
        positional_input
    }
}
