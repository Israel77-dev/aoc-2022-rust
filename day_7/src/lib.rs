#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use regex::Regex;

type NodeList<T> = RefCell<Vec<Rc<T>>>;
#[derive(Debug)]
struct Dir<'a> {
    name: &'a str,
    parent: Option<Weak<Dir<'a>>>,
    subdirs: NodeList<Dir<'a>>,
    files: NodeList<File<'a>>,
}

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    dir: Weak<Dir<'a>>,
    size: usize,
}

impl<'a> Dir<'a> {
    fn get_size(&self) -> usize {
        self.files
            .borrow()
            .iter()
            .fold(0, |acc, file| acc + file.size)
            + self
                .subdirs
                .borrow()
                .iter()
                .fold(0, |acc, dir| acc + dir.get_size())
    }

    fn new(name: &'a str, parent: Option<Weak<Dir<'a>>>) -> Self {
        Dir {
            name: name,
            parent: parent,
            subdirs: RefCell::new(vec![]),
            files: RefCell::new(vec![]),
        }
    }

    fn has_subdirs(&self) -> bool {
        self.subdirs.borrow().len() > 0
    }

    fn has_files(&self) -> bool {
        self.files.borrow().len() > 0
    }
}

fn cd_into<'a>(current: Rc<Dir<'a>>, into: &str) -> Rc<Dir<'a>> {
    let mut target_dir: Rc<Dir> = Rc::new(Dir::new("_", None));
    for dir in current.subdirs.borrow().iter() {
        if dir.name.eq(into) {
            target_dir = Rc::clone(dir);
        }
    }

    target_dir
}

fn cd_out<'a>(current: RefCell<Rc<Dir<'a>>>) -> Rc<Dir<'a>> {
    let parent = current.borrow().parent.as_ref().unwrap().upgrade().unwrap();
    return parent;
}

fn dir_info<'a>(current: RefCell<Rc<Dir<'a>>>, dir_name: &'a str) {
    let new_dir = Dir {
        name: dir_name,
        parent: Some(Rc::downgrade(&*current.borrow())),
        subdirs: RefCell::new(vec![]),
        files: RefCell::new(vec![]),
    };

    current.borrow().subdirs.borrow_mut().push(Rc::new(new_dir));
}

fn file_info<'a>(current: RefCell<Rc<Dir<'a>>>, filename: &'a str, size: usize) {
    let new_file: File<'a> = File {
        name: filename,
        dir: Rc::downgrade(&*current.borrow()),
        size: size,
    };

    current.borrow().files.borrow_mut().push(Rc::new(new_file));
}

fn parse_line<'a>(current: Rc<Dir<'a>>, line: &'a str) -> Rc<Dir<'a>> {
    let re_cd_into = Regex::new(r"\$ cd ([a-z]+)").unwrap();
    let re_cd_out = Regex::new(r"\$ cd (\.\.)").unwrap();
    let re_dir_info = Regex::new(r"dir (.+)").unwrap();
    let re_file_info = Regex::new(r"(\d+) (.+)").unwrap();
    // let re_ls_command = Regex::new(r"\$ ls").unwrap();

    if re_cd_into.is_match(line) {
        let into = get_capture_as_str(&re_cd_into, line, 1);
        // Changes into a new directory
        return Rc::clone(&cd_into(current, into));
    } else if re_cd_out.is_match(line) {
        return Rc::clone(&cd_out(ref_cell_from_rc(&current)));
    } else if re_dir_info.is_match(line) {
        let dir_name = get_capture_as_str(&re_dir_info, line, 1);
        dir_info(ref_cell_from_rc(&current), dir_name);
    } else if re_file_info.is_match(line) {
        let file_size = get_capture_as_str(&re_file_info, line, 1).parse().unwrap();
        let filename = get_capture_as_str(&re_file_info, line, 2);
        file_info(ref_cell_from_rc(&current), filename, file_size);
    }

    // For commands that don't change directories, just return the current one
    current
}

fn get_capture_as_str<'t>(re: &Regex, text: &'t str, n: usize) -> &'t str {
    re.captures(text).unwrap().get(n).unwrap().as_str()
}

fn ref_cell_from_rc<T>(rc: &Rc<T>) -> RefCell<Rc<T>> {
    RefCell::new(Rc::clone(rc))
}

// Adds the total value of subdirs (including the current one) smaller than a given value
fn add_dirs_smaller_than(value: usize, root: Rc<Dir>) -> usize {
    let mut total = 0usize;

    let own_size = root.get_size();
    if own_size < value {
        total += own_size
    }

    if root.has_subdirs() {
        for dir in root.subdirs.borrow().iter() {
            total += add_dirs_smaller_than(value, Rc::clone(dir))
        }
    }

    total
}

fn create_tree_from_input(input: &str) -> Rc<Dir> {
    let root = Rc::new(Dir::new("/", None));

    {
        // Parses the strings to build the tree
        let mut current = Rc::clone(&root);

        for line in input.lines().skip(1) {
            current = parse_line(Rc::clone(&current), line);
        }
    }

    root
}

pub fn process_part_1(input: &str) -> usize {
    let root = create_tree_from_input(input);
    add_dirs_smaller_than(100_000, root)
}

fn min_deletable_dir_size(min_free_space: usize, root: Rc<Dir>) -> usize {
    let mut result = root.get_size();

    if root.has_subdirs() {
        for dir in root.subdirs.borrow().iter() {
            let dir_min = min_deletable_dir_size(min_free_space, Rc::clone(dir));
            if dir_min > min_free_space && dir_min < result {
                result = dir_min
            }
        }
    }
    result
}

pub fn process_part_2(input: &str) -> usize {
    // Free space for my particular solution
    const MIN_FREE_SPACE: usize = 46592386 - 40_000_000;

    let root = create_tree_from_input(input);
    min_deletable_dir_size(MIN_FREE_SPACE, root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parsing_test() {
        let root = Rc::new(Dir {
            name: "/",
            parent: None,
            subdirs: RefCell::new(vec![]),
            files: RefCell::new(vec![]),
        });

        {
            let mut current = Rc::clone(&root);

            current = parse_line(Rc::clone(&current), "100 a.txt");
            current = parse_line(Rc::clone(&current), "dir s");
            current = parse_line(Rc::clone(&current), "$ cd s");
            parse_line(Rc::clone(&current), "1024 name.txt");
            // }
        }
        println!("{:?}", root);
        println!("{}", root.get_size());
        println!("{}", root.subdirs.borrow().get(0).unwrap().get_size());
    }

    #[test]
    fn part_1() {
        const INPUT: &str = "$ cd /
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
7214296 k";

        assert_eq!(process_part_1(INPUT), 95437)
    }

    #[test]
    fn part_2() {
        const INPUT: &str = "$ cd /
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
7214296 k";

        assert_eq!(process_part_2(INPUT), 24933642)
    }
}
