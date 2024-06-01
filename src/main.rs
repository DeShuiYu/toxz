use std::error::Error;
use std::fs::File;
use std::path::PathBuf;
use clap::Parser;
use xz2::write::XzEncoder;

fn create_xz(src_path:&PathBuf,dst_file_path:&PathBuf,level:u32) ->Result<(),Box<dyn Error>>{
    let dst_file = File::create(dst_file_path)?;
    let mut tar = tar::Builder::new(XzEncoder::new(dst_file,level));
    if src_path.is_dir(){
        tar.append_dir_all(".",src_path)?;
    }else if src_path.is_file(){
        tar.append_file(src_path,&mut File::open(src_path)?)?;
    }else{
       return Err(format!("no compress {:?}",src_path).into());
    }
    tar.finish()?;
    return Ok(());
}

#[derive(Parser)]
#[command(version="0.0.1",about="to xz",long_about="use level to xz")]
struct Args{
    /// 源路径，可以是文件或者文件夹
    #[arg(short='i',long="src",required = true)]
    src_path:PathBuf,
    /// 目标路径，必须是文件
    #[arg(short='o',long="dst",required = true)]
    dst_file_path:PathBuf,
    /// 加密等级，默认为6
    #[arg(short='l',long="level",default_value_t=6)]
    level:u32
}

fn main()->Result<(),Box<dyn Error>> {
    let args = Args::parse();
    create_xz(&args.src_path,&args.dst_file_path,args.level)?;
    Ok(())
}
