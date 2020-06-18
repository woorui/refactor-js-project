use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::ffi::OsStr;
use structopt::StructOpt;

/// Replace all file'extension from source_extension to target_extension
#[derive(StructOpt)]
#[structopt(about = "Refactor js project to ts project")]
struct Cli {
    /// The path to the file to read
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: PathBuf,
    /// The source extension, default js
    #[structopt(short = "s", long = "source_extension", default_value = "js")]
    source_extension: String,
    /// The target extension, default ts
    #[structopt(short = "t", long = "target_extension", default_value = "ts")]
    target_extension: String,
}

impl Cli {
    fn change_extension(&self) -> io::Result<i32> {
        change_extension(&self.path, &self.source_extension, &self.target_extension, 0)
    }
}

fn change_extension(dir: &Path, source_extension: &String, target_extension: &String, count: i32) -> io::Result<i32> {
    let mut n = count;
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let m = n;
                change_extension(&path, source_extension, target_extension, m)?;
            }
            let filename = entry.file_name();
            let file_name_str = filename.to_str().expect("file name cant convert to str");
            let splited = file_name_str.split(".").collect::<Vec<&str>>();

            let name = splited[0];
            let extension = path.extension();

            match extension {
                Some(extension) => {
                    if extension == OsStr::new(&source_extension) && !name.is_empty() {
                        println!("count => {}", n);
                        n += 1;
                        let mut target = path.clone();
                        target.set_extension(&target_extension);
                        println!("Will change {:?}, and It's num {}", target, n);
                        // fs::rename(path, target)?;
                    }
                }
                None => {}
            }
        }
    }
    Ok(n)
}
fn main() {
    match Cli::from_args().change_extension() {
        Ok (change_file_count) => println!("The total number of changes was {}", change_file_count),
        Err(e) => eprintln!("It is error that {}", e),
    }
}
