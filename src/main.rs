use regex::Regex;
use std::fs;
use clap::Parser;

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

fn load_file(file_path: &String) -> String {
    fs::read_to_string(file_path).expect("Cannot read file")
}

fn parse_path(path: &String, files: &mut Vec<std::path::PathBuf>) {
    if fs::metadata(path).unwrap().is_file() {
        files.push(
            fs::canonicalize(path)
                .unwrap()
        );
    } else {
        let paths = fs::read_dir(path).unwrap();
        for path_obj in paths {
            parse_path(&path_obj.unwrap().path().to_str().unwrap().to_string(), files)
        }
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

    #[arg(short = 'c', long, default_value_t = false)]
    match_case: bool,
}

fn main() {

    let args = Parameters::parse();

    let regex_pattern = pattern_builder(args.pattern, args.prev_lines, args.next_lines, args.match_case);

    let mut files: Vec<std::path::PathBuf> = Vec::new();
    parse_path(&args.file_path, &mut files);

    for file in files {
        let path_name:String = file.into_os_string().into_string().unwrap();
        let file_contents: String = load_file(&path_name);

        let matched = regex_pattern.find(&file_contents);
    
        if matched.is_some() {
            println!("{} - {}", path_name, matched.unwrap().as_str());
        } else {
            println!("No match!");
        }
    }
}
