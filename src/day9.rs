use std::collections::HashSet;

const INPUT: &'static str = include_str!("../inputs/day9.txt");

#[derive(Debug, Clone, Copy)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

fn parse_input() -> Vec<(Direction, usize)> {
  INPUT
    .lines()
    .map(|line| {
      let dir = match line.chars().next().unwrap() {
        'U' => Direction::Up,
        'D' => Direction::Down,
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => panic!("Invalid direction"),
      };
      let distance = line[2..].parse().unwrap();
      (dir, distance)
    })
    .collect()
}

pub fn solve() {
  let moves = parse_input();

  let mut head_pos = (0isize, 0isize);
  let mut tail_pos = (0isize, 0isize);

  let mut visited_tail_positions = HashSet::new();
  visited_tail_positions.insert(tail_pos);

  for &(direction, distance) in &moves {
    for _ in 0..distance {
      match direction {
        Direction::Up => head_pos.1 += 1,
        Direction::Down => head_pos.1 -= 1,
        Direction::Left => head_pos.0 -= 1,
        Direction::Right => head_pos.0 += 1,
      }

      if head_pos.0 - tail_pos.0 == 2 {
        tail_pos.0 += 1;
        tail_pos.1 = head_pos.1;
      }
      if tail_pos.0 - head_pos.0 == 2 {
        tail_pos.0 -= 1;
        tail_pos.1 = head_pos.1;
      }
      if head_pos.1 - tail_pos.1 == 2 {
        tail_pos.1 += 1;
        tail_pos.0 = head_pos.0;
      }
      if tail_pos.1 - head_pos.1 == 2 {
        tail_pos.1 -= 1;
        tail_pos.0 = head_pos.0;
      }
      visited_tail_positions.insert(tail_pos);
    }
  }

  println!("Part 1: {}", visited_tail_positions.len());

  let mut rope: Vec<(isize, isize)> = vec![(0, 0); 10];
  let mut visited_tail_positions = HashSet::new();
  visited_tail_positions.insert((0, 0));

  for (direction, distance) in moves {
    for _ in 0..distance {
      let head_pos = rope.first_mut().unwrap();
      match direction {
        Direction::Up => head_pos.1 += 1,
        Direction::Down => head_pos.1 -= 1,
        Direction::Left => head_pos.0 -= 1,
        Direction::Right => head_pos.0 += 1,
      }

      for head_ix in 0..(rope.len() - 1) {
        let head_pos = rope[head_ix];
        let tail_ix = head_ix + 1;
        let tail_pos = &mut rope[tail_ix];

        if (head_pos.0 - tail_pos.0).abs() == 2 && head_pos.1 != tail_pos.1
          || (head_pos.1 - tail_pos.1).abs() == 2 && head_pos.0 != tail_pos.0
        {
          let x_dir = (head_pos.0 - tail_pos.0).signum();
          let y_dir = (head_pos.1 - tail_pos.1).signum();
          tail_pos.0 += x_dir;
          tail_pos.1 += y_dir;
        } else {
          if head_pos.0 - tail_pos.0 == 2 {
            tail_pos.0 += 1;
          }
          if tail_pos.0 - head_pos.0 == 2 {
            tail_pos.0 -= 1;
          }
          if head_pos.1 - tail_pos.1 == 2 {
            tail_pos.1 += 1;
          }
          if tail_pos.1 - head_pos.1 == 2 {
            tail_pos.1 -= 1;
          }
        }
      }

      visited_tail_positions.insert(*rope.last().unwrap());
    }
  }

  println!("Part 2: {}", visited_tail_positions.len());
}
