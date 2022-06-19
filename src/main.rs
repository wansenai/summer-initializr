use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::Result;

fn main() -> Result<()> {
   //Command::new("cargo")
  // .arg("new")
   //.arg("hello")
   //.output()
  // .expect("command is failed!");
  fs::create_dir_all("/Users/fenggege/work/opensource/summer-initializr/test/hello/src")?;
  fs::write("/Users/fenggege/work/opensource/summer-initializr/test/hello/src/main.rs",
"fn main () {
   println!(\"hello world\");
}"
   )?;
  fs::write("/Users/fenggege/work/opensource/summer-initializr/test/hello/Cargo.toml"
  ,"
  [package]
  name = \"summer-initializr\"
  version = \"0.1.0\"
  edition = \"2021\"
  
  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
  
  [dependencies]
  cargo = \"0.62\"")?;
  Ok(())
}

pub struct Dir {
   name: String,
   file: File,
}

pub struct Package {
   name: String,
   file: File,
}
