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
pub struct ObjectChars {
    decor: String,
    name: String,
    completed: bool,
    count_attrs: usize,
}

impl ObjectChars {
    fn new() -> ObjectChars {
        ObjectChars {
            completed: false,
            name: String::new(),
            decor: String::new(),
            count_attrs: 0,
        }
    }

    pub fn is_module(&self) -> bool {
        !self.decor.is_empty() && self.decor == "NgModule"
    }

    pub fn decorator(&self) -> Option<&str> {
        if self.decor.is_empty() {
            return None;
        }
        Some(self.decor.as_str())
    }

    pub fn class_name(&self) -> Option<&str> {
        if self.name.is_empty() {
            return None;
        }
        Some(self.name.as_str())
    }

    pub fn count_attrs(&self) -> usize {
        self.count_attrs
    }

    fn is_empty(&self) -> bool {
        self.decor.is_empty() && self.name.is_empty()
    }

    fn is_comlete(&self) -> bool {
        !self.name.is_empty()
    }
}

fn string_is_open(line: &str) -> bool {
    let mut open_by: Option<char> = None;
    let chars: Vec<char> = line.chars().collect();
    for symb in chars {
        if symb == '\'' || symb == '"' {
            if open_by.is_none() {
                open_by = Some(symb);
            } else if open_by.unwrap() == symb {
                open_by = None
            }
        }
    }
    open_by.is_some()
}

// fn mask_str(code: &str) -> String {
//     let mut out: Vec<char> = vec![];
//     let mut open_by: Option<char> = None;
//     // let d = out.iter().collect::Vec<char>().concat();
//     // code.chars().into_iter().map(|s|)
//     let chars: Vec<char> = code.chars().collect();
//     for symb in chars {
//         if symb == '\'' || symb == '"' {
//             if open_by.is_none() {
//                 open_by = Some(symb);
//             } else if open_by.unwrap() == symb {
//                 open_by = None
//             }
//         }

//         if open_by.is_some() {
//             out.push('_');
//         } else {
//             out.push(symb);
//         }
//     }
//     out.into_iter().collect::<String>()
// }

fn clear_comment(mut code: &str) -> String {
    if code.starts_with("/*") || code.starts_with("*") || code.starts_with("//") {
        code = "";
        return code.to_string();
    }
    let parts = code.split("//").collect::<Vec<&str>>();
    let mut out: Vec<&str> = vec![];
    if parts.len() > 1 {
        let mut idx = 0;
        for part in parts {
            if idx == 0 {
                out.push(part);
                continue;
            }
            if string_is_open(part) {
                out.push("//");
                out.push(part);
            } else {
                break;
            }
            idx += 1;
        }
        return out.concat();
    }
    return code.to_string();
}

#[derive(Debug, Clone)]
struct FileCursor {
    elements: Vec<ObjectChars>,
    open: ObjectChars,
    blocks: Vec<char>,
}

impl FileCursor {
    fn new() -> FileCursor {
        FileCursor {
            elements: vec![],
            open: ObjectChars::new(),
            blocks: vec![],
        }
    }

    fn read_line(&mut self, line: &str) {
        let code = &clear_comment(line.trim());
        if code.len() == 0 {
            return;
        }
        if self.is_root() {
            // parse class object
            let re_class =
                Regex::new(r"(?<export>export)?[\s]?class (?<class_name>[\w]+)").unwrap();
            let caps = re_class.captures(code);
            match caps {
                Some(_) => {
                    let class_name = caps.unwrap().name("class_name").unwrap().as_str();
                    self.open.name = class_name.to_string();
                    self.update_blocks(code);
                }
                None => {}
            }

            // parse decorator
            if self.open.is_empty() {
                let re_decor = Regex::new(r"^@(?<decorator>[0-9A-Za-z]*)").unwrap();
                let caps = re_decor.captures(code);

                match caps {
                    Some(_) => {
                        let decor = caps.unwrap().name("decorator").unwrap().as_str();
                        self.open.decor = decor.to_string();
                        self.update_blocks(code);
                    }
                    None => {}
                }
            }
        } else {
            self.check_attributes(code);
            self.update_blocks(code);
        }
    }

    fn check_attributes(&mut self, line: &str) {
        if line == "}" {
            return;
        }
        if self.open.name.is_empty() || self.deep() != 1 || self.blocks[0] != '{' {
            return;
        }
        let attr_re = Regex::new(
            r"^(?<decor>@[0-9A-Za-z\s_]*\(.*\))?(?<name>[\$0-9A-Za-z\s_]+)(<[^>]+>)?[\?\:\=\;\(].*",
        )
        .unwrap();
        let caps = attr_re.captures(line);
        match caps {
            Some(_) => self.open.count_attrs += 1,
            None => {}
        }
    }

    fn deep(&self) -> usize {
        self.blocks.len()
    }

    fn get_open_char(&self, symb: char) -> Option<char> {
        let list_block = [('{', '}'), ('(', ')')];
        for item in list_block {
            if item.1 == symb {
                return Some(item.0);
            }
        }
        return None;
    }

    fn update_blocks(&mut self, line: &str) {
        let chars: Vec<char> = line.chars().collect();
        for symb in chars {
            if symb == '{' || symb == '(' {
                self.blocks.push(symb)
            } else if symb == '}' || symb == ')' {
                let last = self.blocks.last();
                if last.is_some() {
                    let &l = last.unwrap();
                    let open_char = self.get_open_char(symb).unwrap();
                    if l == open_char {
                        // let last_unw
                        self.blocks.pop();
                    } else {
                        dbg!(self);
                        panic!("Open block NOT EQUAL {} != {}", l, symb);
                    }
                } else {
                    panic!("Open block NOT FOUND {}", symb);
                }
            }
        }
        // check component is completed
        if self.open.is_comlete() && self.is_root() {
            self.open.completed = true;
            self.elements.push(self.open.clone());
            self.open = ObjectChars::new();
        }
    }

    fn is_root(&self) -> bool {
        self.blocks.len() == 0
    }
}

#[derive(Debug)]
pub struct FileAnalizResult {
    items: Vec<ObjectChars>,
}

impl FileAnalizResult {
    pub fn count_classes(self) -> usize {
        self.items
            .iter()
            .filter(|x| x.name.is_empty())
            .collect::<Vec<_>>()
            .len()
    }

    pub fn iter_elements(&self) -> std::slice::Iter<'_, ObjectChars> {
        self.items.iter()
    }
}

pub fn read_file(path: &Path) -> FileAnalizResult {
    let mut cursor: FileCursor = FileCursor::new();
    for line in fs::read_to_string(path).unwrap().lines() {
        cursor.read_line(line);
    }
    return FileAnalizResult {
        items: cursor.elements,
    };
}
