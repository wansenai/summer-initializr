
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

fn return_back(package: Package) -> Result<()> {
   let mut version = package.version;
   let name = package.name;
   let mut edition = package.edition;
   let mut denpendecies = package.denpendecies;
   let path = "./".to_string() + &name + "/src";
   fs::create_dir_all(&path)?;
   let main_contents = "fn main () {
      println!(\"hello world\");
   }";
  // fs::write(path.push_str("/main.rs"), main_contents)?;

   let cargo_contents = "[package]\n".to_string()
   + "name = " + "" +&name+"";
   fs::write("./".to_string()+ &name + "/Cargo.toml", cargo_contents)?;
   
   Ok(())
}


