use regex::Regex;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread;
use custom_error::custom_error;


custom_error! {SearchError
    FileNotFound = "File not found",
    IncompatibleFormat = "Format was not parsable"
}

fn search_directory(pattern: &'static str, path: String) -> (u32, u32) {
    let (mut successes, mut failures) = (0, 0);
    let dir = read_dir(path).unwrap();
    let mut jobs: Vec<thread::JoinHandle<Result<bool, SearchError>>>
      = Vec::new();

    for entry in dir {
        let entry_path = entry.unwrap().path();
        if !entry_path.is_dir() {
            let file_path = entry_path.to_str().unwrap().to_string();
            
            jobs.push(thread::spawn(move || {
                search_file(pattern, file_path)
            }));
        } else{
            let (x, y) = search_directory(pattern, 
                                          entry_path.to_str()
                                            .unwrap().to_string());
            successes += x;
            failures += y;
        }
    }

    for i in jobs {
        match i.join(){
            Ok(_) => successes += 1,
            Err(_) => failures += 1
        };
    }

    (successes, failures)
}

fn search_file(pattern: &str, path: String) -> Result<bool, SearchError> {
    // println!("Searching {}", path);
    let mut return_value = true;
    let test = Regex::new(pattern).unwrap();
    let path = Path::new(&path);
    let file = match File::open(path) {
        Ok(x) => x,
        Err(_) => return Err(SearchError::FileNotFound)
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_str = match line{
            Ok(x) => x,
            Err(_) => return Err(SearchError::IncompatibleFormat)
        };
        if test.is_match(&line_str) {
            println!("{}:\t{}", path.display(), line_str);
            return_value = true;
        }
    }

    Ok(return_value)
}


fn main() {
    let outcome = search_directory(r"[Ss]hrek", "./Content/".to_string());
    println!("\nSuccesses:\t{}\nFailures:\t{}", outcome.0, outcome.1);
}
