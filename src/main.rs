
use std::fs;
//use bytes::Bytes;
//use std::io::prelude::*;
use serde_derive::{Deserialize, Serialize};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;
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

async fn parse_request(url: String) -> Result<()> {
   let response = reqwest::get(url)
                                    .await?
                                    .json::<Package>()
                                    .await?;
   
   return_back(response);
   Ok(())
}

async fn return_back(package: Package) -> Result<()> {
   let version = package.version;
   let name = package.name;
   let edition = package.edition;
   let denpendecies = package.denpendecies;
   let path = "./".to_string() + &name + "/src";
   fs::create_dir_all(path.clone())?;
   let main_contents = "fn main () {
      println!(\"hello world\");
   }";
   fs::write(path.clone() + "/main.rs", main_contents)?;

   let cargo_contents = "[package]\n".to_string()
   + "name = " + "" +&name+"";
   fs::write("./".to_string() + &name + "/Cargo.toml",cargo_contents)?;
   
   Ok(())
}

#[tokio::main]
async fn main() {
  // fetch_url("".to_string(),"demo".to_string()).await.unwrap();
  parse_request("http://www.baidu.com".to_string()).await.unwrap();
}

