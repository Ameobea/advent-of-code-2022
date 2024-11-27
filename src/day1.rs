const INPUT: &'static str = include_str!("../inputs/day1.txt");

fn parse_input() -> Vec<usize> {
  let mut sums = Vec::new();
  let mut sum = 0usize;
  for line in INPUT.lines() {
    if line.is_empty() {
      sums.push(sum);
      sum = 0;
      continue;
    }

    let num: usize = line.parse().unwrap();
    sum += num;
  }

  sums.push(sum);
  sums
}

pub fn solve() {
  let mut sums = parse_input();
  let p1 = sums.iter().max().unwrap();
  println!("Part 1: {p1}");

  sums.sort_unstable();
  let top_3 = sums.iter().rev().take(3).collect::<Vec<_>>();
  let p2: usize = top_3.iter().copied().sum();
  println!("Part 2: {p2}");
}
