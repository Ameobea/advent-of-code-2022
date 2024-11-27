use std::collections::VecDeque;

use regex::Regex;

const INPUT: &'static str = include_str!("../inputs/day5.txt");

fn parse_input() -> (Vec<VecDeque<char>>, Vec<(usize, usize, usize)>) {
  let mut stacks: Vec<VecDeque<char>> = Vec::new();
  let mut iter = INPUT.lines().peekable();
  while let Some(line) = iter.next() {
    if matches!(iter.peek(), Some(next_line) if next_line.is_empty()) {
      break;
    }

    let mut col_ix = 0usize;
    loop {
      let char_start_ix = col_ix * 4 + 1;
      let Some(c) = line.chars().nth(char_start_ix) else {
        break;
      };

      while stacks.len() < col_ix + 1 {
        stacks.push(VecDeque::new());
      }

      if c != ' ' {
        stacks[col_ix].push_front(c);
      }
      col_ix += 1;
    }
  }

  // consume the empty line
  assert_eq!(iter.next(), Some(""));

  // parse moves
  let mut moves: Vec<(usize, usize, usize)> = Vec::new();
  let move_rgx = Regex::new("move (\\d+) from (\\d+) to (\\d+)").unwrap();
  while let Some(line) = iter.next() {
    let caps = move_rgx.captures(line).expect("invalid move line");
    let count = caps.get(1).unwrap().as_str().parse().unwrap();
    let from: usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let to: usize = caps.get(3).unwrap().as_str().parse().unwrap();
    moves.push((count, from - 1, to - 1));
  }

  (stacks, moves)
}

pub fn solve() {
  let (orig_stacks, moves) = parse_input();

  let mut stacks = orig_stacks.clone();
  for &(count, from, to) in &moves {
    for _ in 0..count {
      let removed = stacks[from].pop_back().unwrap();
      stacks[to].push_back(removed);
    }
  }

  let top_chars = stacks
    .iter()
    .map(|stack| stack.back().unwrap())
    .collect::<Vec<_>>();
  println!("Part 1: {}", top_chars.into_iter().collect::<String>());

  let mut stacks = orig_stacks.clone();
  for (count, from, to) in moves {
    let mut removed: VecDeque<char> = VecDeque::new();
    for _ in 0..count {
      let c = stacks[from].pop_back().unwrap();
      removed.push_front(c);
    }
    stacks[to].extend(removed);
  }

  let top_chars = stacks
    .iter()
    .map(|stack| stack.back().unwrap())
    .collect::<Vec<_>>();
  println!("Part 2: {}", top_chars.into_iter().collect::<String>());
}
