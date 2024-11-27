const INPUT: &'static str = include_str!("../inputs/day4.txt");

fn parse_input() -> Vec<[[usize; 2]; 2]> {
  // lines are like:
  // 2-8,3-7
  INPUT
    .lines()
    .map(|line| {
      let parts = line.split(',');
      let mut ranges = [[0, 0]; 2];
      for (i, part) in parts.enumerate() {
        let mut range = part.split('-');
        ranges[i][0] = range.next().unwrap().parse().unwrap();
        ranges[i][1] = range.next().unwrap().parse().unwrap();
      }
      ranges
    })
    .collect()
}

fn assignment_fully_contains_other(a0: &[usize; 2], a1: &[usize; 2]) -> bool {
  (a0[0] <= a1[0] && a0[1] >= a1[1]) || (a1[0] <= a0[0] && a1[1] >= a0[1])
}

fn assignment_overlaps_other(a0: &[usize; 2], a1: &[usize; 2]) -> bool {
  (a0[0] <= a1[0] && a0[1] >= a1[0]) || (a1[0] <= a0[0] && a1[1] >= a0[0])
}

pub fn solve() {
  let assignments = parse_input();
  let fully_contained_assignment_count = assignments
    .iter()
    .filter(|[a0, a1]| assignment_fully_contains_other(a0, a1))
    .count();
  println!("Part 1: {fully_contained_assignment_count}");

  // p2
  let overlapping_assignment_count = assignments
    .iter()
    .filter(|[a0, a1]| assignment_overlaps_other(a0, a1))
    .count();
  println!("Part 2: {overlapping_assignment_count}");
}
