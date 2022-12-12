use std::cmp::max;

#[cfg(test)]
mod tests {
    use indoc::indoc;

    const SAMPLE_INPUT: &str = indoc! {"$ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k"};

    #[test]
    fn test_sample_input() {
        assert_eq!(super::size_of_small_dirs(SAMPLE_INPUT), 95437)
    }

    #[test]
    fn smallest_directory_to_delete() {
        assert_eq!(super::smallest_directory_to_delete(SAMPLE_INPUT), 24933642);
    }
}

pub(crate) fn size_of_small_dirs(input: &str) -> u32 {
    let dirs = parse_console_log(input);
    dirs.iter()
        // .inspect(|d| { dbg!(d); })
        .map(|dir| {
            dir.size(&dirs)
        })
        // .inspect(|size| { dbg!(size); })
        .filter(|&size| { size < 100_000 })
        .sum()
}

#[derive(Debug)]
struct DirTree {
    name: String,
    directories: Vec<String>,
    files: Vec<u32>,
}

impl DirTree {
    fn new_sub_directory(&mut self, name: String) -> DirTree {
        let dir_name = if self.name != "/" {
            self.name.clone() + "/" + &name
        } else {
            self.name.clone() + &name
        };
        self.directories.push(dir_name.clone());
        DirTree {
            name: dir_name,
            directories: vec![],
            files: vec![],
        }
    }

    fn add_file(&mut self, size: u32) {
        self.files.push(size);
    }

    pub(crate) fn size(&self, dirs: &[DirTree]) -> u32 {
        dirs.iter()
            .filter(|d| { d.name.starts_with(&self.name) })
            .map(|d| { d.file_size() }).sum()
    }
    fn file_size(&self) -> u32 {
        self.files.iter().sum()
    }
}

fn parse_console_log(input: &str) -> Vec<DirTree> {
    let mut dirs: Vec<DirTree> = Vec::new();
    let mut cwd = String::new();
    for line in input.lines() {
        if line.starts_with("$ cd") {
            let cd = &line[5..];
            if cd == ".." {
                cwd.truncate(max(1, cwd.rfind('/').unwrap()));
            } else if cd == "/" {
                dirs.push(DirTree {
                    name: "/".to_string(),
                    directories: vec![],
                    files: vec![],
                });
                cwd = String::from("/");
            } else {
                let idx = dirs.iter().position(|d| { d.name == cwd }).unwrap();
                let sub_dir = dirs[idx].new_sub_directory(cd.to_string());
                cwd = sub_dir.name.clone();
                dirs.push(sub_dir);
            }
        } else if line.starts_with("$ ls") || line.starts_with("dir") {
        } else {
            let size: u32 = line.split_whitespace().next().map(str::parse).unwrap().unwrap();
            let idx = dirs.iter().position(|d| { d.name == cwd }).unwrap();
            dirs[idx].add_file(size);
        }
    }
    dirs
}

static FS_SPACE: u32 = 70_000_000;
static REQUIRED_SPACE: u32 = 30_000_000;
pub(crate) fn smallest_directory_to_delete(input: &str) -> u32 {
    let dirs = parse_console_log(input);
    let free_space = FS_SPACE - dirs.first().unwrap().size(&dirs);
    let space_to_free = REQUIRED_SPACE - free_space;
    dirs.iter()
        .map(|d| { d.size(&dirs)})
        .filter(|&d| { d > space_to_free })
        .min()
        .unwrap()
}
