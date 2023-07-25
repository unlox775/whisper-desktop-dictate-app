use std::process::Command;
use std::str;

fn main() {
    let commands = ["rec", "whisper", "pbcopy", "killall", "cat"];

    for command in &commands {
        let command_path = Command::new("which")
            .arg(command)
            .output()
            .expect("failed to execute process");

        let command_path = str::from_utf8(&command_path.stdout)
            .expect("not UTF-8")
            .trim();

        // pass the path to the rustc compiler
        println!("cargo:rustc-env={}_PATH={}", command.to_uppercase(), command_path);
    }
}
