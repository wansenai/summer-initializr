
use std::{
   fs,
   fs::File,
   io::{Read, Write, Seek},
   process::Command,
   path::Path,
   //net::TcpListener,
   //thread,
};
use zip::result::ZipResult;
use zip::write::{FileOptions, ZipWriter};
use serde_derive::{Deserialize, Serialize};
use walkdir::{DirEntry, WalkDir};
use summer_boot::{Request, Result};
use summer_boot::log::{info};

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
   command_generation_rust_project(&name);
   //初始化项目文件路径
   let path = Path::new(&name);
   let mut init_zip_path = String::new();
   init_zip_path.push_str(&name);
   init_zip_path.push_str(".zip");
   let zip_path = Path::new(&mut init_zip_path);
   compress_dir(path, zip_path);
   //删除文件保留zip文件
   fs::remove_dir_all(path);
   let mut file = File::open(zip_path).unwrap();
   let mut io_byte = Vec::new();
   file.read_to_end(&mut io_byte);
   Ok(format!("hello,{:?}", &mut io_byte).into())
}

fn command_generation_rust_project(filename: &str) {
    Command::new("cargo")
    .arg("new")
    .arg(filename)
    .output()
    .expect("cargo new failed");
}

///压缩文件夹
fn compress_dir(src_dir: &Path, target: &Path){
    let zip_file = File::create(target).unwrap();
    let dir = WalkDir::new(src_dir);
    //文件压缩
    zip_dir(&mut dir.into_iter().filter_map(|e|e.ok()),
     src_dir.to_str().unwrap(), zip_file);
}

///压缩文件文件夹方法
fn zip_dir<T>(it: &mut dyn Iterator<Item = DirEntry>, prefix: &str, writer: T) -> 
    ZipResult<()> 
        where T: Seek + Write
{
    let mut zip = ZipWriter::new(writer);
    let options = FileOptions::default()
                              .compression_method(zip::CompressionMethod::Bzip2)//Bzip2格式
                              .unix_permissions(0o755);//unix权限
    let mut buf = Vec::new();
    for entry in it {
        let path = entry.path();
        //zip压缩一个文件时，会把它的全路径当成文件名
        //去掉目录前缀
        info!("prefix:{:?}...", path);
        let name = path.strip_prefix(Path::new(prefix)).unwrap();
        info!("name:{:?}...", name);
        //如果是文件
        if path.is_file() {
            info!("add file {:?} as {:?}..", path, name);
            zip.start_file_from_path(name, options)?;
            let mut f = File::open(name)?;
            //文件读取
            f.read_to_end(&mut buf)?;
            zip.write_all(&*buf);
            buf.clear();
            //如果是目录
        } 
        //else if name.as_os_str().len() != 0 {
           // println!("add dir {:?} as {:?} ...", path, name);
           // zip.add_directory_from_path(name, options)?;
        //}
    }
    zip.finish()?;
     Ok(())
}



