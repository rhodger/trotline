use regex::Regex;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread;
use custom_error::custom_error;
use clap::{App, Arg};


custom_error! {SearchError
    FileNotFound = "File not found",
    IncompatibleFormat = "Format was not parsable"
}

fn search_directory(pattern: String, path: String, nocase: bool) -> (u32, u32) {
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
            let pattern_copy = pattern.to_string();
            jobs.push(thread::spawn(move || {
                search_file(pattern_copy, file_path, nocase)
            }));
        } else {
            // For every directory, recursively call search_directory() on it.
            let pattern_copy = pattern.to_string();
            let (x, y) = search_directory(pattern_copy, 
                                          entry_path.to_str()
                                            .unwrap().to_string(),
                                          nocase);

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

fn search_file(pattern: String, path: String, nocase: bool) -> Result<bool, SearchError> {
    // Use return_value to record whether this function successfully processed
    // the given file or not.
    let mut return_value = true;

    let pattern_slice = &pattern[..];
    let test = Regex::new(pattern_slice).unwrap();
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
        let mut comparison_str = line_str.to_string();
        if nocase { comparison_str = comparison_str.to_lowercase() }
        if test.is_match(&comparison_str) {
            println!("{}:\t{}", path.display(), line_str);
            return_value = true;
        }
    }

    Ok(return_value)
}


fn main() {
    // Handle command-line arguments
    let matches = App::new("trotline")
                    .version("1.1.0")
                    .author("Suede G")
                    .about("Simplified grep clone")
                    .arg(Arg::with_name("pattern")
                           .required(true)
                           .help("regex search pattern")
                           .index(1)
                           .takes_value(true))
                    .arg(Arg::with_name("directory")
                           .index(2)
                           .help("target directory")
                           .takes_value(true))
                    .arg(Arg::with_name("nocase")
                           .help("ignore case")
                           .short("i")
                           .long("ignore_case")
                           .takes_value(false))
                    .get_matches();

    let mut nocase = false;
    let mut pattern = matches.value_of("pattern").unwrap().to_string();

    if matches.occurrences_of("nocase") > 0 {
        pattern = pattern.to_lowercase();
        nocase = true;
    }

    // If no directory was given at command-line, use current working directory
    // as default.
    let directory = matches.value_of("directory").unwrap_or("./").to_string();

    let outcome = search_directory(pattern, directory, nocase);
    println!("\nSuccesses:\t{}\nFailures:\t{}", outcome.0, outcome.1);

}
