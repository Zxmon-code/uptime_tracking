use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{fs, path::PathBuf};

fn main() {
    let files = get_all_files();
    let lines = get_all_line_numbers();
    graph_all(files, lines);
}

fn calculate_weekday_from_date_string(date_string: &str) -> String {
    let date_parts: Vec<&str> = date_string.split("_").collect();
    let year = date_parts[0].parse::<i32>().unwrap();
    let month = date_parts[1].parse::<i32>().unwrap();
    let day = date_parts[2].parse::<i32>().unwrap();
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let year = if month < 3 { year - 1 } else { year };
    let weekday =
        ((year + year / 4 - year / 100 + year / 400 + t[(month - 1) as usize] + day) % 7) as usize;
    let weekdays = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    weekdays[weekday].to_string()
}
fn get_bar_length(len: i32) -> String {
    let mut final_string = "".to_string();
    for i in 0..(len / 10) {
        if i % 6 == 0 && i != 0 {
            final_string.push('|')
        }
        final_string.push('#');
    }
    final_string
}

fn graph_all(files: Vec<String>, lines: Vec<i32>) {
    let mut all_lengths = 0;
    for entry in 0..files.len() {
        let file_name: Vec<&str> = files[entry].split("/").collect();
        print!(
            "{}: {} | {}",
            file_name[file_name.len() - 1],
            format!(
                "{: <9}",
                calculate_weekday_from_date_string(file_name[file_name.len() - 1])
            ),
            get_bar_length(lines[entry])
        );
        println!();
        all_lengths += lines[entry];
    }
    println!("Average in minutes: {}", all_lengths / lines.len() as i32)
}

fn get_all_line_numbers() -> Vec<i32> {
    let files = get_all_files();
    let mut all_lines = vec![1];
    all_lines.remove(0);
    for file in files {
        all_lines.push(get_number_of_lines_for_file(file));
    }
    // print!("{:?}", all_lines);
    all_lines
}

fn get_number_of_lines_for_file(file: String) -> i32 {
    let file = File::open(file).expect("could not open file");
    let buffer = BufReader::new(file);
    let line_count = buffer.lines().count() as i32;
    // print!("{}", line_count);
    line_count
}

fn get_all_files() -> Vec<String> {
    // get HOME env variable for uptime directory
    let home_dir = String::from(std::env::var("HOME").unwrap());
    let home_dir: Vec<&str> = home_dir.split('"').collect();
    let home_dir: String = home_dir[0].to_string();
    // make uptime_dir variable using HOME env var
    let uptime_dir = format!("{}/uptime", home_dir);
    let files: Result<Vec<PathBuf>, std::io::Error> = fs::read_dir(uptime_dir)
        .unwrap()
        .map(|result| result.map(|f| f.path()))
        .collect();
    let mut files: Vec<PathBuf> = files.unwrap();
    files.sort();
    let mut files_string: Vec<String> = vec!["".to_string()];
    for file in files {
        files_string.push(file.as_path().display().to_string())
    }
    files_string.retain(|f| *f != "".to_string());
    // println!("{:?}", files_string);
    files_string
}
