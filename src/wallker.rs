use std::{ffi::OsStr, fs, path::PathBuf};

use crate::*;

pub fn read_path(path: &PathBuf) -> DirStat {
    let p = path.as_path();
    let mut stat = DirStat::new();

    if p.is_file() {
        let extension = path.extension().and_then(OsStr::to_str);
        // Skip all not .ts files
        if extension.is_none() || extension.unwrap() != "ts" {
            return stat;
        }

        debug!("* Read .TS file: {}", path.display());
        stat.count_ts += 1;
        let file_result = file_reader::read_file(path);

        file_result
            .iter_elements()
            .filter(|elem| elem.class_name().is_some())
            .for_each(|elem| {
                // stat.add_class(elem.class_name().unwrap().to_string());
                stat.count_class += 1;
                stat.count_attr += elem.count_attrs();
            });
        return stat;
    } else if p.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let path_to_file = file.unwrap().path();
            stat.add(read_path(&path_to_file))
        }
    }
    stat
}
