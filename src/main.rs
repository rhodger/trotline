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
    // Record how many times `search_file` succeeded.
    let (mut successes, mut failures) = (0, 0);

    let dir = read_dir(path).unwrap();

    // Holds a handle for each spawned thread so they can be joined later.
    let mut jobs: Vec<thread::JoinHandle<Result<bool, SearchError>>>
      = Vec::new();

    for entry in dir {
        let entry_path = entry.unwrap().path();

        if !entry_path.is_dir() {
            // For every file, create a thread running search_file() on it.
            let file_path = entry_path.to_str().unwrap().to_string();
            
            jobs.push(thread::spawn(move || {
                search_file(pattern, file_path)
            }));
        } else {
            // For every directory, recursively call search_directory() on it.
            let (x, y) = search_directory(pattern, 
                                          entry_path.to_str()
                                            .unwrap().to_string());

            // Combine this instance of search_directory()'s results with those
            // of the recursive call that just completed.
            successes += x;
            failures += y;
        }
    }

    for i in jobs {
        // Join every spawned thread here and record whether they succeeded in
        // processing the given file or not.
        match i.join(){
            Ok(_) => successes += 1,
            Err(_) => failures += 1
        };
    }

    (successes, failures)
}

fn search_file(pattern: &str, path: String) -> Result<bool, SearchError> {
    // Use return_value to record whether this function successfully processed
    // the given file or not.
    let mut return_value = true;

    let test = Regex::new(pattern).unwrap();
    let path = Path::new(&path);
    let file = match File::open(path) {
        Ok(x) => x,
        Err(_) => return Err(SearchError::FileNotFound)
    };
    let reader = BufReader::new(file);

    for line in reader.lines() {
        // Attempt to process and run regex search on each line of the given
        // file.
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
    // Regex pattern and target directory are currently hardcoded, however this
    // function is designed in such a way that implementing custom values is
    // trivial once CLI arguments are being parsed.
    let outcome = search_directory(r"[Ss]hrek", "./Content/".to_string());
    println!("\nSuccesses:\t{}\nFailures:\t{}", outcome.0, outcome.1);
}
