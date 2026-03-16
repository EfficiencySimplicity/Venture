use text_io;

// https://users.rust-lang.org/t/lifetime-issue-when-reading-file-as-string-split-into-words/39516/3
pub fn get_command() -> Vec<String> {
    let input: String = text_io::read!("{}\n");
    let command = Vec::from_iter(input.split_whitespace().map(str::to_string));
    command
}

// For testing purposes
pub fn print_command() {
    let command = get_command();
    println!("{}", command.join(" "));
}