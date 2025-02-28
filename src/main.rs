use std::{env, fs};

fn main() {
    let arg_path = env::args().nth(1).expect("No path given!!!");

    let path = std::path::PathBuf::from(arg_path);

    if !path.exists() {
        panic!("The given path cannot be found!!!");
    }

    let tmp_files = fs::read_dir(path).unwrap();


    for file in tmp_files {
        let entry = file.unwrap();

        println!("file in path : {:?}", entry.path());
    }
}
