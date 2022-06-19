use std::process::Command;

fn main() {
   Command::new("cargo")
   .arg("new")
   .arg("hello")
   .output()
   .expect("command is failed!");
}
