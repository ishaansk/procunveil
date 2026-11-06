
use std::any::Any;
use std::fs;
use std::path::PathBuf;

fn main(){
    let mut procpath = PathBuf::new();
    procpath.push("/proc/");
    let entries = fs::read_dir(procpath).expect("failed to open directory");
    println!("{:?}",entries);
    let mut procdirs = Vec::new();
    for entry in entries{
        match entry{
            Ok(entry) =>{
                let filename = entry.file_name().to_string_lossy().into_owned();
                //let metadata = entry.metadata();
                //println!("{}, {:?}",filename,metadata);
                if let Ok(metadata) = entry.metadata(){
                    if metadata.is_dir(){
                        procdirs.push(filename);
                    }
                }
            }
            Err(error)=>{
                eprintln!("Error loading entry {}",error);
            }
        }
    }
  println!("{:?}",procdirs)
}
