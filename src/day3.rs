use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/day3.txt");

fn parse_input() -> Vec<[Vec<char>; 2]> {
  let mut rucksack_contents = Vec::new();

  for line in INPUT.lines() {
    let char_count = line.len();
    let compartment_size = char_count / 2;

    let (compartment_1, compartment_2) = line.split_at(compartment_size);
    assert_eq!(compartment_1.len(), compartment_2.len());

    rucksack_contents.push([
      compartment_1.chars().collect(),
      compartment_2.chars().collect(),
    ]);
  }

  rucksack_contents
}

fn priority(c: char) -> usize {
  if c.is_lowercase() {
    c as usize - 'a' as usize + 1
  } else {
    c as usize - 'A' as usize + 27
  }
}

pub fn solve() {
  assert_eq!(priority('a'), 1);
  assert_eq!(priority('A'), 27);
  assert_eq!(priority('z'), 26);
  assert_eq!(priority('Z'), 52);

  let rucksack_contents = parse_input();

  let mut prioritites_sum = 0usize;
  for [c0, c1] in &rucksack_contents {
    let c0_set: HashSet<char> = HashSet::from_iter(c0.iter().copied());
    let c1_set: HashSet<char> = HashSet::from_iter(c1.iter().copied());

    let common_chars = c0_set.intersection(&c1_set);
    prioritites_sum += common_chars
      .into_iter()
      .copied()
      .map(priority)
      .sum::<usize>();
  }

  println!("Part 1: {}", prioritites_sum);

  // p2
  let mut badge_priority_sum = 0usize;
  for [chunk0, chunk1, chunk2] in rucksack_contents.array_chunks() {
    // need to find the character common between all 3 chunks
    let c0_set: HashSet<char> =
      HashSet::from_iter(chunk0[0].iter().chain(chunk0[1].iter()).copied());
    let c1_set: HashSet<char> =
      HashSet::from_iter(chunk1[0].iter().chain(chunk1[1].iter()).copied());
    let c2_set: HashSet<char> =
      HashSet::from_iter(chunk2[0].iter().chain(chunk2[1].iter()).copied());

    let common_chars = c0_set
      .intersection(&c1_set)
      .copied()
      .collect::<HashSet<_>>()
      .intersection(&c2_set)
      .copied()
      .collect::<HashSet<_>>();
    assert_eq!(common_chars.len(), 1);
    let common_char = common_chars.into_iter().next().unwrap();
    badge_priority_sum += priority(common_char);
  }

  println!("Part 2: {}", badge_priority_sum);
}
