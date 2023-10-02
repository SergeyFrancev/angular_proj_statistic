pub struct DirStat {
    pub count_ts: i32,
    pub count_class: usize,
    pub count_attr: usize,
    class_list: Vec<String>,
}

impl DirStat {
    pub fn new() -> DirStat {
        DirStat {
            count_ts: 0,
            count_class: 0,
            count_attr: 0,
            class_list: vec![],
        }
    }

    pub fn add(&mut self, stat: DirStat) {
        self.count_ts += stat.count_ts;
        self.count_class += stat.count_class;
        self.count_attr += stat.count_attr;
        // self.class_list.concat();
        self.class_list.extend(stat.class_list)
    }

    pub fn add_class(&mut self, cl_name: String) {
        if !self.class_list.contains(&cl_name) {
            self.class_list.push(cl_name)
        }
    }

    pub fn count_classes(&self) -> usize {
        self.class_list.len()
    }

    pub fn count_attr(&self) -> usize {
        self.count_attr.clone()
    }
}
