use std::{fs, path::Path};

use clap::ValueEnum;
use regex::Regex;

#[derive(ValueEnum, Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Class,
    Component,
    Module,
    Pipe,
    No,
}

#[derive(Debug, Clone)]
struct ObjectChars {
    is_module: bool,
    is_component: bool,
    is_pipe: bool,
    selector: String,
    name: String,
}

impl ObjectChars {
    pub fn new() -> ObjectChars {
        ObjectChars {
            is_module: false,
            is_component: false,
            is_pipe: false,
            selector: String::new(),
            name: String::new(),
        }
    }

    fn is_empty(self) -> bool {
        self.is_module == false && self.is_component == false && self.is_pipe == false
        //  && self.name.is_none()
    }
}

const MAX_ELEMENTS: usize = 10;
#[derive(Debug, Clone)]
struct FileCursor {
    elements: Vec<ObjectChars>,
    open: ObjectChars,
    blocks: Vec<char>,
}

impl FileCursor {
    fn clone(&self) -> Self {
        return FileCursor {
            elements: self.elements.clone(),
            open: ObjectChars::new(),
            blocks: self.blocks.clone(),
        };
    }

    fn new() -> FileCursor {
        FileCursor {
            elements: vec![],
            open: ObjectChars::new(),
            blocks: vec![],
        }
    }

    fn read_line(mut self, line: &str) {
        let code = line.trim();
        if code.len() == 0 {
            return;
        }
        if self.blocks.len() == 0 {
            if let idx = code.find("@NgModule") {
                // self.open = Some(ObjectChars::new());
                self.open.is_module = true;
                self.update_blocks(code, idx)
            }
        }
    }
    fn update_blocks(mut self, line: &str, start: Option<usize>) {
        let mut chars: Vec<char> = line.chars().collect();
        if start.is_some() {
            chars = chars.splice(0..start.unwrap(), []).collect()
        }
        for symb in chars {
            if symb == '{' || symb == '(' {
                self.blocks.push(symb)
            } else if symb == '}' || symb == ')' {
                let last = self.blocks.last();
                if last.is_some() {
                    let &l = last.unwrap();
                    if l == symb {
                        // let last_unw
                        self.blocks.pop();
                    } else {
                        panic!("Open block NOT EQUAL {} != {}", l, symb);
                    }
                } else {
                    panic!("Open block NOT FOUND {}", symb);
                }
            }
        }
    }

    fn is_root(self) -> bool {
        self.blocks.len() == 0
    }
}

#[derive(Debug)]
pub struct FileAnalizResult {
    items: Vec<ObjectChars>,
}

impl FileAnalizResult {
    fn new() -> FileAnalizResult {
        FileAnalizResult { items: vec![] }
    }

    pub fn count_classes(self) -> usize {
        self.items
            .iter()
            .filter(|x| x.name.is_empty())
            .collect::<Vec<_>>()
            .len()
    }

    // pub fn class_names(&mut self) -> Vec<String> {
    //     self.items.iter().filter(|x| x.name.is_some()).collect::<Vec<_>>()
    // }
}

pub fn read_file(path: &Path) -> FileAnalizResult {
    let mut result = FileAnalizResult::new();
    let re_class = Regex::new(r"export class (?<class_name>[\w]+)").unwrap();
    let cursor: FileCursor = FileCursor::new();
    // let c = &cursor;
    // fs::read_to_string(path).unwrap().lines().for_each(|f|cursor.read_line(f.to_string()));
    for line in fs::read_to_string(path).unwrap().lines() {
        // result.push(line.to_string())
        // cursor.blocks.push('1');
        // cursor.blocks.push('{');
        cursor.read_line(line);
        let caps = re_class.captures(line);
        match caps {
            Some(_) => {
                let class_name = caps.unwrap().name("class_name").unwrap().as_str();
                let mut obj_char = ObjectChars::new();
                obj_char.name = class_name.to_string();
                result.items.push(obj_char);
                // stat.add_class(class_name.to_string());
                // stat.count_class += 1;
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
    result
}
