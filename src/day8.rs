const INPUT: &'static str = include_str!("../inputs/day8.txt");

fn parse_input() -> Vec<Vec<usize>> {
  INPUT
    .lines()
    .map(|line| {
      line
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect()
    })
    .collect()
}

fn get_scenic_score(tree_heights: &[Vec<usize>], x: usize, y: usize) -> usize {
  let height = tree_heights.len();
  let width = tree_heights[0].len();

  let base_height = tree_heights[y][x];

  // looking up
  let mut visible_tree_count_up = 0usize;
  for y_ix in (0..y).rev() {
    if y_ix == y {
      continue;
    }

    let height = tree_heights[y_ix][x];
    visible_tree_count_up += 1;
    if height >= base_height {
      break;
    }
  }

  // looking down
  let mut visible_tree_count_down = 0usize;
  for y_ix in y..height {
    if y_ix == y {
      continue;
    }

    let height = tree_heights[y_ix][x];
    visible_tree_count_down += 1;
    if height >= base_height {
      break;
    }
  }

  // looking left
  let mut visible_tree_count_left = 0usize;
  for x_ix in (0..x).rev() {
    if x_ix == x {
      continue;
    }

    let height = tree_heights[y][x_ix];
    visible_tree_count_left += 1;
    if height >= base_height {
      break;
    }
  }

  // looking right
  let mut visible_tree_count_right = 0usize;
  for x_ix in x..width {
    if x_ix == x {
      continue;
    }

    let height = tree_heights[y][x_ix];
    visible_tree_count_right += 1;
    if height >= base_height {
      break;
    }
  }

  visible_tree_count_up
    * visible_tree_count_down
    * visible_tree_count_left
    * visible_tree_count_right
}

pub fn solve() {
  let tree_heights = parse_input();
  let height = tree_heights.len();
  let width = tree_heights[0].len();
  let mut visible_flags = vec![vec![false; width]; height];

  // looking left->right
  for y_ix in 0..height {
    let x_ix = 0;
    visible_flags[y_ix][x_ix] = true;

    let mut max_height = tree_heights[y_ix][x_ix];
    for x_ix in 0..width {
      let height = tree_heights[y_ix][x_ix];
      if height > max_height {
        max_height = height;
        visible_flags[y_ix][x_ix] = true;
      }
    }
  }

  // looking right->left
  for y_ix in 0..height {
    let x_ix = width - 1;
    visible_flags[y_ix][x_ix] = true;

    let mut max_height = tree_heights[y_ix][x_ix];
    for x_ix in (0..width).rev() {
      let height = tree_heights[y_ix][x_ix];
      if height > max_height {
        max_height = height;
        visible_flags[y_ix][x_ix] = true;
      }
    }
  }

  // looking top->bottom
  for x_ix in 0..width {
    let y_ix = 0;
    visible_flags[y_ix][x_ix] = true;

    let mut max_height = tree_heights[y_ix][x_ix];
    for y_ix in 0..height {
      let height = tree_heights[y_ix][x_ix];
      if height > max_height {
        max_height = height;
        visible_flags[y_ix][x_ix] = true;
      }
    }
  }

  // looking bottom->top
  for x_ix in 0..width {
    let y_ix = height - 1;
    visible_flags[y_ix][x_ix] = true;

    let mut max_height = tree_heights[y_ix][x_ix];
    for y_ix in (0..height).rev() {
      let height = tree_heights[y_ix][x_ix];
      if height > max_height {
        max_height = height;
        visible_flags[y_ix][x_ix] = true;
      }
    }
  }

  let visible_count = visible_flags.iter().flatten().filter(|&&flag| flag).count();
  println!("Part 1: {visible_count}");

  // dbg!(get_scenic_score(&tree_heights, 2, 3));
  let mut max_scenic_score = 0usize;
  for y_ix in 0..height {
    for x_ix in 0..width {
      max_scenic_score = max_scenic_score.max(get_scenic_score(&tree_heights, x_ix, y_ix));
    }
  }
  println!("Part 2: {max_scenic_score}");
}
