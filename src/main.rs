
use std::{
   fs::File,
   io::{Read, Write, Seek},
   path::Path,
};
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};
use serde_derive::{Deserialize, Serialize};
use summer_boot::{Request, Result};

///包名对象
#[derive(Deserialize, Serialize)]
pub struct Package {
   name: String,
   version: Option<String>,
   edition: Option<String>,
   denpendecies: Option<Vec<Dependency>>,
}
///依赖对象
#[derive(Deserialize, Serialize)]
pub struct Dependency {
   name: Option<String>,
   version: Option<String>,
}

#[summer_boot::auto_scan]
#[summer_boot::main]
async fn main() {
   summer_boot::run();
}

#[summer_boot::post("/test/api")]
async fn test_api(mut req: Request<()>) -> Result {
   let Package {
      name,
      version,
      edition,
      denpendecies
   } = req.body_json().await?;
   //初始化项目文件路径
   let mut init_zip_path = String::new();
   init_zip_path.push_str(&name);
   init_zip_path.push_str(".zip");
   let mut file = File::create(&mut init_zip_path).expect("not exist");
   create_zip_archive(&mut file);
   let zip_path = Path::new(&mut init_zip_path);
   //删除文件保留zip文件
   let mut zip_file = File::open(zip_path).unwrap();
   let mut io_byte = Vec::new();
   zip_file.read_to_end(&mut io_byte);
   Ok(format!("hello,{:?}", &mut io_byte).into())
}

static APP_YML_FILE: &'static [u8] = include_bytes!("../src/resources/application.yml");
static APP_TEST_YML_FILE: &'static [u8] = include_bytes!("../src/resources/application-test.yml");
static GITIGNORE: &'static [u8] = include_bytes!("../.gitignore");
///创建zip文件并且写文件
/// todo！拼接名称，libs，bins包名
fn create_zip_archive<T: Seek + Write>(buf: &mut T) -> ZipResult<()> {
   let mut writer = ZipWriter::new(buf);
   writer.start_file("demo/src/resources/application.yml", FileOptions::default())?;
   writer.write(APP_YML_FILE);
   writer.start_file("demo/src/resources/application-test.yml", FileOptions::default())?;
   writer.write(APP_TEST_YML_FILE);
   writer.start_file("demo/.gitignore", FileOptions::default())?;
   writer.write(GITIGNORE);
   writer.start_file("demo/src/main.rs", FileOptions::default())?;
   writer.write(b"
   use serde::Deserialize;
   use summer_boot::{Request, Result};
   
   #[derive(Debug, Deserialize)]
   struct User {
       name: String,
       age: u16,
   }
   
   #[summer_boot::auto_scan]
   #[summer_boot::main]
   async fn main() {
       summer_boot::run();
   }
   
   #[summer_boot::post(\"/test/api\")]
   async fn test_api(mut req: Request<()>) -> Result {
       let User { name, age } = req.body_json().await?;
       Ok(format!(\"Hello, {}!  {} years old\", name, age).into())
   }
   ");
   writer.start_file("demo/Cargo.toml", FileOptions::default())?;
   writer.write(b"
   [package]
   name = \"demo\"
   version = \"0.1.0\"
   edition = \"2021\"
   
   # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
   
   [dependencies]
   summer-boot = \"1.0.0\"
   serde = \"1.0.137\"
   serde_json = \"1.0.81\"
   serde_derive = \"1.0.137\"
   ");
   writer.finish()?;
   Ok(())
}



