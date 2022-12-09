use std::{fs::read_to_string};

struct Directory {
    name: String,
    files: Vec<File>,
    parent: Option<usize>,
    children: Vec<usize>,
    total_size: usize
}

#[derive(PartialEq)]
struct File {
    name: String,
    size: usize
}

impl File {
    fn parse(line: &str) -> File {
        let (size, name) = line.split_once(" ").unwrap();
        File { name: name.to_string(), size: size.parse().unwrap() }
    }
}

struct FileSystem {
    curdir: usize,
    directories: Vec<Directory>,
}

impl FileSystem{

    fn new(root: Directory) -> FileSystem {
        FileSystem { curdir: 0, directories: vec![root] }
    }

    pub fn change_dir(&mut self, dir: &str) {
        match dir {
            ".." => {
                let curdir = &self.directories[self.curdir];
                if let Some(parent) = curdir.parent {
                   self.curdir = parent;
                }
            },
            "/" => self.curdir = 0,
            dir => {
                let curdir = &self.directories[self.curdir];
                for node in &curdir.children {
                    if self.directories[*node].name == dir {
                        self.curdir = *node;
                        return;
                    }
                }
                let idx = self.directories.len();
                let new_dir = Directory {
                    name: dir.to_string(),
                    children: Vec::new(),
                    files: Vec::new(),
                    parent: Option::Some(self.curdir),
                    total_size: 0
                };
                self.directories.push(new_dir);
                self.curdir = idx;
            }
        }
    }

    pub fn list_dir(&mut self, contents: Vec<&str>) {
        let mut curdir = &mut self.directories[self.curdir];
        let files: Vec<File> = contents.iter()
        .filter(|c| !c.starts_with("dir"))
        .map(|l| File::parse(l))
        .collect();
        let mut size: usize = 0;
        for file in files {
            if !curdir.files.contains(&file) {
                size += file.size;
                curdir.files.push(file);
            }
        }
        curdir.total_size += size;
        let mut curdir_idx = curdir.parent;
        while curdir_idx.is_some()  {
            let mut curdir = &mut self.directories[curdir_idx.unwrap()];
            curdir.total_size += size;
            curdir_idx = curdir.parent;
        }
    }

    fn apply_command(&mut self, command: &str, contents: Vec<&str>) {
        if command.starts_with("cd") {
            let (_, dir) = command.split_once(" ").unwrap();
            self.change_dir(dir);
        } else if command.starts_with("ls") {
            self.list_dir(contents)
        } else {
            panic!("Invalid command")
        }
    }
}

fn apply_log(log: &str, filesystem: &mut FileSystem) {
    let lines: Vec<&str> = log.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i];
        if line.starts_with("$") {
            let command = &line[2..line.len()];
            let next_cmd_index = lines[i+1..lines.len()].iter()
            .position(|l| l.starts_with("$"))
            .and_then(|a| Some(a + i + 1)).unwrap_or(lines.len());
            let output = if next_cmd_index > i + 1 {
                lines[i+1..next_cmd_index].to_vec()
            } else {
                Vec::new()
            };
            filesystem.apply_command(command, output);
            i = next_cmd_index;
        } 
    }
}


fn main() {
    let input = read_to_string("./input/task_1.txt").unwrap();
    let root = Directory {
        name: String::from("/"),
        files: Vec::new(),
        children: Vec::new(),
        parent: Option::None,
        total_size: 0
    };

    let mut filesystem = FileSystem::new(root);
    apply_log(&input, &mut filesystem);

    let task_1_answer: usize = filesystem.directories.iter()
    .map(|d| d.total_size)
    .filter(|s| *s <= 100000 as usize)
    .sum();
    println!("[Task 1] Sum of total sizes of at most 100000: {}", task_1_answer);

    let available = 70000000 - filesystem.directories[0].total_size;
    let to_be_freed = 30000000 - available;
    let task_2_answer = filesystem.directories.iter()
    .map(|dir| dir.total_size)
    .filter(|&size| size >= to_be_freed)
    .min()
    .unwrap();
    println!("[Task 2] Size of min directory to be deleted: {}", task_2_answer);
}