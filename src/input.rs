pub fn is_positional(input: &[String]) -> bool {
    input[0] != "-"
}

pub fn is_stdin(input: &[String]) -> bool {
    !is_positional(input)
}

pub fn is_valid(input: &[String]) -> bool {
    (is_stdin(input) && grep_cli::is_readable_stdin()) || (!is_stdin(input) && is_positional(input))
}

pub fn parse(args: Vec<String>) -> Vec<String> {
    use std::io::BufRead;

    match is_stdin(&args) && grep_cli::is_readable_stdin() {
        true => {
            let stdin = std::io::stdin();

            let mut stdin_input: Vec<String> = Vec::new();
            for line in stdin.lock().lines() {
                let unquoted_input = line.unwrap().replace('"', "").to_owned();
                stdin_input.push(unquoted_input);
            }

            stdin_input
        }
        false => args,
    }
}
