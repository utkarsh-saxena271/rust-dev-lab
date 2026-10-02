use std::fs::DirEntry;
use std::fs;
use std::path::PathBuf;
use std::env;


fn main() {

    // terminal input
    let args:Vec<String> = env::args().collect();
    println!("{:?}", args);

    let inp_path;

    // got input path
    if args.len() <= 2 || args[2] == "." {
       inp_path = env::current_dir().unwrap()
    } else {
        inp_path = PathBuf::from(&args[2])
    };

    println!("{:?}", inp_path);

    // dir content result
    let dir_content_result = fs::read_dir(inp_path);


    // got dir content from result into a vector
    let dir_content: Vec<DirEntry> = match dir_content_result {
        Ok(entries) => {
            let mut entries_vec: Vec<DirEntry> = Vec::new();

            for entry in entries {
                match entry {
                    Ok(val) => entries_vec.push(val),
                    Err(err) => println!("Error reading entry: {}", err),
                }
            }

            entries_vec
        }

        Err(err) => {
            println!("Error reading directory: {}", err);
            Vec::new()
        }
    };

    println!("{:?}", dir_content);
}
