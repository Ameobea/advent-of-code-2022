use slotmap::{Key, SlotMap};

slotmap::new_key_type! {
  pub struct DirKey;
}

const INPUT: &str = include_str!("../inputs/day7.txt");

struct File {
  #[allow(dead_code)]
  name: String,
  size: usize,
}

struct Dir {
  name: String,
  subdir_ids: Vec<DirKey>,
  files: Vec<File>,
  parent_id: DirKey,
}

type DirMap = SlotMap<DirKey, Dir>;

impl Dir {
  pub fn new(dirs: &mut DirMap, name: String, parent_id: DirKey) -> DirKey {
    dirs.insert(Self {
      name,
      subdir_ids: Vec::new(),
      files: Vec::new(),
      parent_id,
    })
  }
}

fn parse_input() -> (DirMap, DirKey) {
  let mut lines = INPUT.lines();
  assert_eq!(lines.next().unwrap(), "$ cd /");

  let mut dirs: DirMap = SlotMap::with_key();
  let root_dir_key = Dir::new(&mut dirs, "/".to_string(), DirKey::null());

  let mut current_dir_key = root_dir_key;
  let mut is_listing = false;
  for line in lines {
    if line == "$ ls" {
      assert!(!is_listing);
      is_listing = true;
      continue;
    }

    if line.starts_with("dir ") {
      let dir_name = line.split_whitespace().nth(1).unwrap();
      let dir_id = Dir::new(&mut dirs, dir_name.to_string(), current_dir_key);
      dirs[current_dir_key].subdir_ids.push(dir_id);
      continue;
    } else if line.starts_with("$ cd ") {
      let target_dir_name = line.split("$ cd ").nth(1).unwrap();
      let target_dir = if target_dir_name == ".." {
        dirs[current_dir_key].parent_id
      } else {
        *dirs[current_dir_key]
          .subdir_ids
          .iter()
          .find(|&&dir_key| dirs[dir_key].name == target_dir_name)
          .unwrap()
      };
      current_dir_key = target_dir;
      is_listing = false;
      continue;
    } else if line == "$ cd .." {
      current_dir_key = dirs[current_dir_key].parent_id;
      is_listing = false;
      continue;
    }

    let mut spl = line.split_whitespace();
    let (size, fname) = (spl.next().unwrap(), spl.next().unwrap());

    let size: usize = size.parse().unwrap();
    dirs[current_dir_key].files.push(File {
      name: fname.to_string(),
      size,
    });
  }

  (dirs, root_dir_key)
}

/// Gets the size of all files in directory and its subdirectories recursively.
fn get_dir_size_recursive(
  dirs: &DirMap,
  dir_key: DirKey,
  dir_sizes: &mut Vec<(DirKey, usize)>,
) -> usize {
  let mut size = 0usize;
  for file in &dirs[dir_key].files {
    size += file.size;
  }

  for subdir_key in &dirs[dir_key].subdir_ids {
    size += get_dir_size_recursive(dirs, *subdir_key, dir_sizes);
  }

  dir_sizes.push((dir_key, size));
  size
}

pub fn solve() {
  let (dir_entries_by_id, root_dir_key) = parse_input();
  let mut dir_sizes = Vec::new();
  get_dir_size_recursive(&dir_entries_by_id, root_dir_key, &mut dir_sizes);
  let p1 = dir_sizes
    .iter()
    .filter(|&&(_key, size)| size <= 100000)
    .map(|(_key, size)| size)
    .sum::<usize>();
  println!("Part 1: {p1}");

  let available_space = 70000000;
  let needed_free_space = 30000000;
  dir_sizes.sort_unstable_by_key(|&(_key, size)| size);
  assert_eq!(dir_sizes.last().unwrap().0, root_dir_key);
  let cur_free_space = available_space - dir_sizes.last().unwrap().1;

  let size_of_dir_to_delete = dir_sizes
    .into_iter()
    .find(|&(_key, size)| cur_free_space + size >= needed_free_space)
    .unwrap()
    .1;
  println!("Part 2: {size_of_dir_to_delete}");
}
