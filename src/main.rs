
use std::{
   fs,
   io::{Read, Write},
   net::TcpListener,
   thread,
};
use serde_derive::{Deserialize, Serialize};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() {
   let listener = TcpListener::bind("").unwrap();
   loop {
      let (mut stream, addr) = listener.accept().unwrap();
      println!("Accepted a new connection: {}", addr);
      thread::spawn(move || {
         let mut buf = [0; std::mem::size_of::<usize>()];
         stream.read_exact(&mut buf).unwrap();
         let json_size = usize::from_be_bytes(buf);
         let mut json_buf = vec![0; json_size];
         stream.read(&mut json_buf).unwrap();
         let value: Package = serde_json::from_slice(&json_buf).unwrap();
         //let data = String::from_utf8_lossy(&buf).to_string();
         //println!("data:{:?}", serde_json::from_str(&data));
         stream.write_all(b"").unwrap();
      });
   }
}
///包名对象
#[derive(Deserialize, Serialize)]
pub struct Package {
   name: String,
   version: String,
   edition: String,
   denpendecies: Vec<Dependency>,
}
///依赖对象
#[derive(Deserialize, Serialize)]
pub struct Dependency {
   name: String,
   version: String,
}

fn write_buf(package: Package) -> Result<()> {
   let version = package.version;
   let name = package.name;
   let edition = package.edition;
   let denpendecies = package.denpendecies;
   let path = "./".to_string() + &name + "/src";
   fs::create_dir_all(&path)?;
   let main_contents = "fn main () {
      println!(\"hello world\");
   }";
   fs::write("./".to_string() + &name + "/src/main.rs", main_contents)?;
   let mut cargo_content = String::new();
   cargo_content.push_str("[package]\n");
   cargo_content.push_str("name = ");
   cargo_content.push_str("\"");
   cargo_content.push_str(&name);
   cargo_content.push_str("\"");
   cargo_content.push_str("\nedition = ");
   cargo_content.push_str("\"");
   cargo_content.push_str(&edition);
   cargo_content.push_str("\"");
   cargo_content.push_str("\nversion = ");
   cargo_content.push_str("\"");
   cargo_content.push_str(&version);
   cargo_content.push_str("\"");
   cargo_content.push_str("\n[dependcies]");
   for d in &denpendecies {
      cargo_content.push_str(d.name.as_str());
      cargo_content.push_str(" = ");
      cargo_content.push_str("\"");
      cargo_content.push_str(d.version.as_str());
      cargo_content.push_str("\"");
   };
   fs::write("./".to_string()+ &name + "/Cargo.toml", cargo_content)?;
   
   Ok(())
}


