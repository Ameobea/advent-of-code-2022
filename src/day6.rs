use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/day6.txt");

pub fn solve() {
  let chars = INPUT.chars().collect::<Vec<_>>();
  for (i, window) in chars.array_windows::<4>().enumerate() {
    let char_set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
    if char_set.len() == 4 {
      println!("Part 1: {}", i + 4);
      break;
    }
  }

  // part 2
  for (i, window) in chars.array_windows::<14>().enumerate() {
    let char_set: HashSet<char> = HashSet::from_iter(window.iter().cloned());
    if char_set.len() == 14 {
      println!("Part 2: {}", i + 14);
      break;
    }
  }
}
