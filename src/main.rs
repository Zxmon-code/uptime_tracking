use std::{fs, path::PathBuf};

fn main() {
    get_all_files();
    print!("{:?}", std::env::var("HOME"))
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

    println!("{:?}", files_string);

    vec!["".to_string()]
}
