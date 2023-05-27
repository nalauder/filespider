use clap::Parser;
use regex::Regex;
use std::{fs, path::PathBuf};

fn pattern_builder(
    string_to_match: String,
    prev_lines: u32,
    next_lines: u32,
    case: bool,
) -> regex::Regex {
    let case_flag = if case { "(?i)" } else { "" };
    let search_pattern = format!(
        "{case_flag}(?:.*\\n){{0,{prev_lines}}}.*{string_to_match}.*(?:\\n.*){{0,{next_lines}}}"
    );

    Regex::new(&search_pattern).expect("Cannot create regular expression!")
}

fn load_file(file_path: &PathBuf, verbose: bool) -> Result<String, std::io::Error> {
    let file: String = match fs::read_to_string(file_path) {
        Ok(file) => file,
        Err(error) => {
            if verbose {
                eprintln!(
                    "Error loading file \"{}\" - {}",
                    file_path.to_str().unwrap(),
                    error.to_string()
                );
            }
            return Err(error);
        }
    };
    Ok(file)
}

fn identify_files_local(path: PathBuf, regex_pattern: &Regex, print_line_num: bool, verbose: bool) {
    let file: fs::Metadata = fs::metadata(&path).expect("Cannot read file metadata!");

    if file.is_file() {
        let full_path = fs::canonicalize(path).expect("Cannot canonicalize path!");
        match_in_file(full_path, regex_pattern, print_line_num, verbose);
    } else if file.is_dir() {
        match fs::read_dir(&path) {
            Ok(paths) => {
                for path_obj in paths {
                    identify_files_local(path_obj.expect("Cannot read directory children!").path(), regex_pattern, print_line_num, verbose)
                }
            }
            Err(error) => {
                if verbose {
                    eprintln!(
                        "Error reading directory \"{}\" - {}",
                        path.to_str().unwrap(),
                        error.to_string()
                    );
                }
            }
        }
    }

}

fn match_in_file(file: PathBuf, regex_pattern: &Regex, print_line_num: bool, verbose: bool) {
    let path_name = file.to_str().unwrap();

    let file_contents = load_file(&file, verbose);

    if file_contents.is_ok() {
        for (i, l) in file_contents.unwrap().lines().enumerate() {
            let matched: Option<regex::Match> = regex_pattern.find(l);

            if matched.is_some() {
                let match_obj = matched.unwrap();
                if print_line_num {
                    println!("{}:{} - {}", path_name, i + 1, match_obj.as_str());
                } else {
                    println!("{} - {}", path_name, match_obj.as_str());
                }
            }
        }
    }
}

#[derive(Parser)]
struct Parameters {
    #[arg(short, long)]
    file_path: PathBuf,
    #[arg(short, long)]
    pattern: String,

    #[arg(short = 'A', long, default_value_t = 0)]
    prev_lines: u32,

    #[arg(short = 'B', long, default_value_t = 0)]
    next_lines: u32,

    #[arg(short = 'i', default_value_t = false, action)]
    ignore_case: bool,

    #[arg(short = 'n', default_value_t = false, action)]
    print_line_number: bool,

    #[arg(short = 'v', default_value_t = false, action)]
    verbose: bool,
}

fn main() {
    let args: Parameters = Parameters::parse();

    let regex_pattern: Regex = pattern_builder(
        args.pattern,
        args.prev_lines,
        args.next_lines,
        args.ignore_case,
    );

    identify_files_local(args.file_path, &regex_pattern, args.print_line_number, args.verbose);
}
