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

    Regex::new(&search_pattern).unwrap()
}

fn load_file(file_path: &String) -> Result<String, std::io::Error> {
    let file = fs::read_to_string(file_path)?;
    Ok(file)
}

fn parse_path(path: &String, files: &mut Vec<std::path::PathBuf>) {
    let file = fs::metadata(path);
    if file.is_ok() {
        if file.unwrap().is_file() {
            files.push(fs::canonicalize(path).unwrap());
        } else {       
            match fs::read_dir(path) {
                Ok(paths) => {
                    for path_obj in paths {
                        parse_path(
                            &path_obj.unwrap().path().to_str().unwrap().to_string(),
                            files,
                        )
                    }
                },
                Err(_) => println!("Error opening file - {}", path),
            }
        }
    } else {
        println!("File not found - {}", path);   
    }
}

fn match_in_file(file: PathBuf, regex_pattern: &Regex, print_line: bool) {
    let path_name: String = file.into_os_string().into_string().unwrap();
    
    match load_file(&path_name) {
        Ok(file_contents) => {
            for (i, l) in file_contents.lines().enumerate() {
                let matched: Option<regex::Match> = regex_pattern.find(l);
        
                if matched.is_some() {
                    let match_obj = matched.unwrap();
                    if print_line {
                        println!("{}:{} - {}", path_name, i+1, match_obj.as_str());
                    } else {
                        println!("{} - {}", path_name, match_obj.as_str());
                    }
                    
                }
            }
        },
        Err(_) => return,
    }

    
}

#[derive(Parser)]
struct Parameters {
    #[arg(short, long)]
    file_path: String,
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
}

fn main() {
    let args: Parameters = Parameters::parse();

    let regex_pattern: Regex = pattern_builder(
        args.pattern,
        args.prev_lines,
        args.next_lines,
        args.ignore_case,
    );

    let mut files: Vec<std::path::PathBuf> = Vec::new();
    parse_path(&args.file_path, &mut files);

    for file in files {
        match_in_file(file, &regex_pattern, args.print_line_number);
    }
}
