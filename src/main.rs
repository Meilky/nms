use regex::Regex;
use std::{env, fs, path::PathBuf};

fn list(path: PathBuf) -> () {
    let artist_rg = Regex::new(r"^(?P<artist>.+?)\s-").unwrap();

    let files = fs::read_dir(path).unwrap();

    for file in files {
        let entry = file.unwrap().file_name();
        let file_name = entry.to_str().unwrap();

        let caps = artist_rg.captures(file_name).unwrap();

        println!("Artist: {:?}", &caps["artist"]);
    }
}

fn main() {
    let arg_operation = env::args().nth(1).expect("No operation given!!!");
    let arg_path = env::args().nth(2).expect("No path given!!!");

    let path = std::path::PathBuf::from(arg_path);

    if !path.exists() {
        panic!("The given path cannot be found!!!");
    }

    match arg_operation.as_str() {
        "list" => list(path),
        _ => panic!("Unknown operation {}!!!", arg_operation),
    };
}
