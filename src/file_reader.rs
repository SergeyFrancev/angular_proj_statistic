#[derive(Debug)]
struct FileCursor {
    blocks: Vec<char>,
}

#[derive(Debug)]
struct ObjectChars {
    is_module: bool,
    is_component: bool,
    is_pipe: bool,
    selector: String,
    name: String,
}

#[derive(Debug)]
struct FileAnalizResult {
    classes: Vec<String>,
}
