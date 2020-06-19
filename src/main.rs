use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::fs;
use glob::glob;

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
    fn change_extension_in_glob(&self) -> i32  {
        let mut count: i32 = 0;
        let path = Path::new(&self.path).join("**/*".to_owned() + &self.source_extension);
        let pattern = path.to_str().expect("Parse pattern error");
        for entry in glob(pattern).expect("Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    count += 1;
                    let mut target = path.clone();
                    target.set_extension(&self.target_extension);
                    println!("Will change {}, and It's No {}", target.display(), count);
                    fs::rename(path, target).expect("Failed to rename")
                }
                Err(e) => println!("{:?}", e),
            }
        };
        count
    }
}

fn main() {
    println!("The total number of changes was {}", Cli::from_args().change_extension_in_glob());
}
