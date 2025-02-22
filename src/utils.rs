use dialoguer::Select;
use std::io::{self, Write};

pub fn read_line(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn select_from_list<T: AsRef<str> + ToString>(prompt: &str, items: &[T]) -> usize {
    Select::new()
        .with_prompt(prompt)
        .items(items)
        .interact()
        .unwrap()
}
