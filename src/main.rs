
use std::{
   fs::File,
   io::{Read, Write, Seek},
   path::Path,
};
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};
use serde_derive::{Deserialize, Serialize};
use summer_boot::{Request, Result};
use summer_boot::log::{info, warn, error};
use serde_json::*;

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

#[summer_boot::get("/start.zip")]
async fn start_zip(mut req: Request<()>) -> Result {
   let body = req.body_string().await?;
   //初始化项目文件路径
   let package: Package = serde_json::from_str(&body).unwrap();
   let mut init_zip_path = String::new();
   init_zip_path.push_str(format!("{}.zip",&package.name).as_str());
   let mut file = File::create(&mut init_zip_path).expect("not exist");
   create_zip_archive(&mut file, &package);
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
fn create_zip_archive<T: Seek + Write>(buf: &mut T, package: &Package) -> ZipResult<()> {
   let mut writer = ZipWriter::new(buf);
   writer.start_file(format!("{}/src/resources/application.yml", package.name), FileOptions::default())?;
   writer.write(APP_YML_FILE);
   writer.start_file(format!("{}/src/resources/application-test.yml", package.name), FileOptions::default())?;
   writer.write(APP_TEST_YML_FILE);
   writer.start_file(format!("{}/.gitignore", package.name), FileOptions::default())?;
   writer.write(GITIGNORE);
   writer.start_file(format!("{}/src/main.rs", package.name), FileOptions::default())?;
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
   writer.start_file(format!("{}/Cargo.toml", package.name), FileOptions::default())?;
   let mut main_rs_str = String::new();
   main_rs_str.push_str(format!("[package]\nname = \"{}\"\n", package.name).as_str());
   main_rs_str.push_str(format!("version = \"{}\"\n", package.version.as_ref().unwrap_or(&"0.1.0".to_string())).as_str());
   main_rs_str.push_str(format!("edition = \"{}\"\n", package.edition.as_ref().unwrap_or(&"2021".to_string())).as_str());
   main_rs_str.push_str("\n# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html\n");
   main_rs_str.push_str("\n[dependencies]\nsummer-boot = \"1.0.0\"\nserde = \"1.0.137\"\nserde_json = \"1.0.81\"\nserde_derive = \"1.0.137\"\n");
   let list = Vec::new();
   let dependencies = package.denpendecies.as_ref().unwrap_or(&list);
   if dependencies.len() > 0 {
        for pkg in dependencies {
            let name = pkg.name.as_ref().unwrap();
            let version = pkg.version.as_ref().unwrap();
            main_rs_str.push_str(format!("{} = {} \n", &name, &version).as_str());
        }
   }
   writer.write(main_rs_str.as_bytes());
   writer.start_file(format!("{}/src/service/mod.rs", package.name), FileOptions::default())?;
   writer.start_file(format!("{}/src/domain/mod.rs", package.name), FileOptions::default())?;
   writer.start_file(format!("{}/src/api/mod.rs", package.name), FileOptions::default())?;
   writer.finish()?;
   Ok(())
}



