pub mod file_reader;

use std::{
    env,
    ffi::OsStr,
    fs::{self},
    path::{Path, PathBuf},
};

use regex::Regex;

struct DirStat {
    count_ts: i32,
    count_class: i32,
    class_list: Vec<String>,
}

impl DirStat {
    fn new() -> DirStat {
        DirStat {
            count_ts: 0,
            count_class: 0,
            class_list: vec![],
        }
    }
    fn add(&mut self, stat: DirStat) {
        self.count_ts += stat.count_ts;
        self.count_class += stat.count_class;
        // self.class_list.concat();
        self.class_list.extend(stat.class_list)
    }
    fn add_class(&mut self, cl_name: String) {
        if !self.class_list.contains(&cl_name) {
            self.class_list.push(cl_name)
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parth_to_dir = &args[1];

    use std::time::Instant;
    let now = Instant::now();
    // let path = String::from(parth_to_dir);
    let dir = Path::new(parth_to_dir).as_os_str();
    // let stat = file_stat(dir);
    let stat = read_dir(dir);
    println!("Count .ts files: {}", stat.count_ts);
    println!("Count CLASS in files: {}", stat.count_class);
    println!("Count CLASS: {}", stat.class_list.len());
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn read_dir(parth_to_dir: &OsStr) -> DirStat {
    // let mut count_ts = 0;
    let mut stat = DirStat::new();
    for file in fs::read_dir(parth_to_dir).unwrap() {
        let path: PathBuf = file.unwrap().path();
        // println!("- [{}]", file.display());

        if path.is_dir() {
            // let dir_stat = read_dir(path.as_os_str());
            stat.add(read_dir(path.as_os_str()));
            // return stat
            continue;
        }

        let extension = path.extension().and_then(OsStr::to_str);
        match extension {
            Some("ts") => {
                stat.count_ts += 1;
                stat.add(file_stat(path.as_os_str()))
            }
            Some(_e) => {
                continue;
            }
            None => {
                continue;
            }
        }
        // count_ts += 1;
        // println!("Name: {}", path.display());
        // if path.is_dir() { continue; }
        // transform_to_html(&path);
    }
    stat
}

fn file_stat(file_path: &OsStr) -> DirStat {
    let mut stat = DirStat::new();
    // let re_class = Regex::new(r"export class (?<class_name>[\w]+)").unwrap();
    stat.add(read_by_line(&file_path));
    stat
}

fn read_by_line(file_path: &OsStr) -> DirStat {
    let mut stat = DirStat::new();
    let re_class = Regex::new(r"export class (?<class_name>[\w]+)").unwrap();
    for line in fs::read_to_string(file_path).unwrap().lines() {
        // result.push(line.to_string())
        let caps = re_class.captures(line);
        match caps {
            Some(_) => {
                let class_name = caps.unwrap().name("class_name").unwrap().as_str();
                stat.add_class(class_name.to_string());
                stat.count_class += 1;
                continue;
            }
            None => {
                continue;
            }
        }
        // if re_class.is_match(line) {
        //     stat.add_class(class_name.to_string());
        //     stat.count_class += 1;
        // }
    }
    return stat;
}

fn capture_all_content(file_path: &OsStr) -> DirStat {
    let mut stat = DirStat::new();
    let text: String = fs::read_to_string(&file_path).expect("Incorrect file path");
    let re_class = Regex::new(r"export class (?<class_name>[\w]+)").unwrap();
    for (_, [class_name]) in re_class.captures_iter(&text).map(|c| c.extract()) {
        // if cl_name == "for" {
        //     println!("Class: {}", file_path.to_str().unwrap());
        // }
        stat.add_class(class_name.to_string());
        stat.count_class += 1;
    }
    return stat;
}
// fn transform_to_html(file_path: &PathBuf) {
// 	println!("* transform: {}", file_path.display());
// 	let text: String = fs::read_to_string(&file_path).expect("Incorrect file path");
// 	let html = text_to_html(text);
// 	create_html_file(file_path, html);
// }

// fn create_html_file(file_path: &PathBuf, content: String) {
// 	let path_to_src: String = file_path.clone().into_os_string().into_string().unwrap();
//     let file_name: &str = file_path.file_name().unwrap().to_str().unwrap();
// 	let new_name = file_name.replace("txt", "html");
// 	let path_to_dist: String = path_to_src.replace(file_name, new_name.as_str());
// 	fs::write(path_to_dist.clone(), content).expect("Can't write to file");
// 	println!("- [{}]", path_to_dist);
// }

// fn text_to_html(text: String) -> String {
// 	let mut html: String = text.split("\r\n")
// 		.filter(|part| part.len() > 0)
// 		.map(|part| format!("<p>{}</p>", part))
// 		.collect();
// 	html = format!("<div>{}</div>", html);
// 	html
// }
