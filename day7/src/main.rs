use advent22::*;

#[derive(Debug)]
enum FileType {
    Dir { name: String, children: Vec<usize> },
    File { name: String, size: usize },
}

impl FileType {
    fn name(&self) -> &String {
        match self {
            FileType::Dir { name, .. } => name,
            FileType::File { name, .. } => name,
        }
    }

    /// Returns `true` if the file type is [`Dir`].
    ///
    /// [`Dir`]: FileType::Dir
    #[must_use]
    fn is_dir(&self) -> bool {
        matches!(self, Self::Dir { .. })
    }
}

#[derive(Debug)]
struct File {
    parent: Option<usize>,
    typ: FileType,
}

// idea taken from
// https://rust-leipzig.github.io/architecture/2016/12/20/idiomatic-trees-in-rust/
#[derive(Debug)]
struct FileTree {
    nodes: Vec<File>,
    cur: usize,
}

impl FileTree {
    fn size(&self, idx: usize) -> usize {
        match &self.nodes[idx].typ {
            FileType::Dir { children, .. } => {
                children.iter().map(|&c| self.size(c)).sum()
            }
            FileType::File { size, .. } => *size,
        }
    }

    fn new() -> Self {
        Self {
            nodes: vec![File {
                parent: None,
                typ: FileType::Dir {
                    name: "/".to_owned(),
                    children: Vec::new(),
                },
            }],
            cur: 0,
        }
    }
    /// change to a directory if it exists, or create it and cd to it
    fn cd(&mut self, dir: &str) {
        if dir == "/" {
            self.cur = 0;
            return;
        }
        match &self.nodes[self.cur].typ {
            FileType::Dir { children, .. } => {
                if dir == ".." {
                    self.cur = self.nodes[self.cur].parent.unwrap();
                    return;
                }
                for child in children {
                    if self.nodes[*child].typ.name() == dir {
                        // cd to that child
                        self.cur = *child;
                        return;
                    }
                }
            }
            FileType::File { .. } => todo!(),
        }
        // didn't find the child, create it
        self.nodes.push(File {
            parent: Some(self.cur),
            typ: FileType::Dir {
                name: dir.to_owned(),
                children: Vec::new(),
            },
        });
        let n = self.nodes.len();
        if n == 1 {
            return;
        }
        match &mut self.nodes[self.cur].typ {
            FileType::Dir {
                ref mut children, ..
            } => {
                children.push(n - 1);
            }
            _ => todo!(),
        }
        self.cur = self.nodes.len() - 1;
    }

    fn add_file(&mut self, f: &str, size: usize) {
        let n = self.nodes.len();
        self.nodes.push(File {
            parent: Some(self.cur),
            typ: FileType::File {
                name: f.to_owned(),
                size,
            },
        });
        let FileType::Dir{ children, .. } = &mut self.nodes[self.cur].typ else {
	    panic!()
	};
        children.push(n);
    }
}

fn part1(file_tree: &FileTree) -> usize {
    let mut tot = 0;
    for (i, node) in file_tree.nodes.iter().enumerate() {
        if node.typ.is_dir() {
            let size = file_tree.size(i);
            if size <= 100000 {
                tot += size;
            }
        }
    }
    tot
}

fn part2(file_tree: &FileTree) -> usize {
    const MAX: usize = 70000000;
    const NEED: usize = 30000000;
    let cur_used = file_tree.size(0);
    let cur_avail = MAX - cur_used;
    let want = NEED - cur_avail;
    let mut dirs: Vec<_> = file_tree
        .nodes
        .iter()
        .enumerate()
        .flat_map(|(i, node)| {
            if node.typ.is_dir() {
                let size = file_tree.size(i);
                if size >= want {
                    Some(size)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();
    dirs.sort();
    dirs[0]
}

fn main() {
    let s = load_input();
    let mut in_ls = false;
    let mut file_tree = FileTree::new();
    for line in s.lines() {
        if line.starts_with('$') {
            let mut sp = line.split_ascii_whitespace();
            let cmd = sp.nth(1).unwrap();
            match cmd {
                "cd" => {
                    let dir = sp.next().unwrap();
                    file_tree.cd(dir);
                }
                "ls" => in_ls = true,
                _ => panic!("unrecognized command {cmd}"),
            }
        } else if in_ls && !line.starts_with("dir") {
            let sp: Vec<_> = line.split_ascii_whitespace().collect();
            let size = sp[0].parse().unwrap();
            file_tree.add_file(sp[1], size);
        }
    }
    let p1 = part1(&file_tree);
    let p2 = part2(&file_tree);
    assert_eq!(p1, 1555642);
    assert_eq!(p2, 5974547);
    println!("{}", p1);
    println!("{}", p2);
}
