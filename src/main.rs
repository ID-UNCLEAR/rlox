use std::env;
use std::env::Args;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path_string: String = get_path_argument();
    let path: &Path = Path::new(&path_string);
    read_file(path)?;

    Ok(())
}

fn get_path_argument() -> String {
    let mut args: Args = env::args();
    while let Some(arg) = args.next() {
        if arg == "--path" {
            return args.next()
                .expect("No value provided for `--path` argument!");
        }
    }

    panic!("Required `--path` argument not provided!");
}

fn read_file(path: &Path) -> io::Result<()> {
    let file: File = File::open(path)?;
    let reader: BufReader<File> = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        println!("{}. `{}`", index + 1, line?);
    }

    Ok(())
}
