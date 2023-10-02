pub mod dir_stat;
pub mod file_reader_test;

use std::{
    env,
    ffi::OsStr,
    fs::{self},
    path::{Path, PathBuf},
};

use dir_stat::DirStat;
use file_reader_test::read_file;

// struct Args {
//     /// Name of the person to greet
//     #[arg(short, long)]
//     name: String,

//     /// Number of times to greet
//     #[arg(short, long, default_value_t = 1)]
//     count: u8,
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let parth_to_dir = &args[1];

    use std::time::Instant;
    let now = Instant::now();
    // let path = String::from(parth_to_dir);
    let dir = Path::new(parth_to_dir);
    // let stat = file_stat(dir);
    let stat = read_path(dir);
    println!("Count .ts files: {}", stat.count_ts);
    println!("Count CLASS in files: {}", stat.count_class);
    println!("Count CLASS: {}", stat.count_classes());
    println!("Count ATTRS: {}", stat.count_attr);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn read_path(parth: &Path) -> DirStat {
    // let mut count_ts = 0;
    let mut stat = DirStat::new();

    let item = Path::new(parth);
    if item.is_file() {
        let extension = item.extension().and_then(OsStr::to_str);
        match extension {
            Some("ts") => {
                stat.count_ts += 1;
                // println!("- {}", parth.display());
                let file_result = read_file(parth);

                for elem in file_result.iter_elements() {
                    match elem.class_name() {
                        Some(_) => {
                            if elem.decorator().is_none() || (elem.decorator().is_some() && elem.decorator().unwrap() != "NgModule")
                            {
                                // println!("- {}", elem.count_attrs());
                                stat.add_class(elem.class_name().unwrap().to_string());
                                stat.count_attr += elem.count_attrs();
                            }
                        }
                        None => {}
                    }
                    // stat.add_class(elem.class_name())
                }
                // stat.count_class += file_result.count_classes();
                // stat.add(file_stat(item.as_os_str()))
            }
            Some(_e) => {}
            None => {}
        }
        return stat;
    } else if item.is_dir() {
        for file in fs::read_dir(parth).unwrap() {
            let path: PathBuf = file.unwrap().path();
            stat.add(read_path(path.as_path()))
        }
    }
    stat
}
