use std::path::{Path, PathBuf};
use structopt::StructOpt;
use std::{fs, process};
use glob::glob;
use std::error::Error;


/// Replace all file'extension from source_extension to target_extension
#[derive(StructOpt)]
#[structopt(about = "Refactor js project to ts project")]
struct Cli {
    /// The path to the file to read
    #[structopt(short = "p", long = "path", parse(from_os_str))]
    path: PathBuf,
    /// The source extension
    #[structopt(short = "s", long = "source_extension", default_value = "js")]
    source_extension: String,
    /// The target extension
    #[structopt(short = "t", long = "target_extension", default_value = "ts")]
    target_extension: String,
}

impl Cli {
    fn change_extension_in_glob(&self) -> Result<i32, Box<dyn Error>>  {
        let mut count: i32 = 0;
        let path = Path::new(&self.path).join("**/*".to_owned() + &self.source_extension);
        let pattern = path.to_str().expect("Failed to parse glob pattern");
        for entry in glob(pattern)? {
            let p = entry?;
            count += 1;
            let mut target = p.clone();
            target.set_extension(&self.target_extension);
            println!("Will change {}, and It's No {}", target.display(), count);
            fs::rename(p, target)?
        };
        Ok(count)
    }
}

fn main() {
    let count = Cli::from_args().change_extension_in_glob().unwrap_or_else(|e| {
        eprintln!("Refactor error {}", e);
        process::exit(1);
    });
    println!("The total number of changes was {}", count);
}
