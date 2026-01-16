use iced::widget::{button, column, text};
use std::io::{self, Write};
use std::process::Command;

fn listdir() {
    let list_dir = Command::new("bash")
        .arg("-c")
        .arg("ls")
        .arg("-lah")
        .output()
        .expect("failed to execute process");

    let _ = io::stdout().write(&list_dir.stdout);
}
fn main() {
    println!("Hello World!");
}
