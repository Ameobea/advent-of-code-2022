const INPUT: &'static str = include_str!("../inputs/day10.txt");

#[derive(Debug)]
enum Istn {
  Noop,
  Addx(isize),
}

fn parse_input() -> Vec<Istn> {
  INPUT
    .lines()
    .map(|line| {
      if line == "noop" {
        Istn::Noop
      } else {
        let immediate = line[5..].parse::<isize>().unwrap();
        Istn::Addx(immediate)
      }
    })
    .collect()
}

fn print_screen(screen: Vec<Vec<bool>>) {
  let mut lines = Vec::new();
  for line in screen {
    let mut out_line = String::new();
    for c in line {
      out_line.push(if c { '#' } else { ' ' });
    }
    lines.push(out_line);
  }

  for line in lines {
    println!("{line}");
  }
}

pub fn solve() {
  let istns = parse_input();

  let mut x = 1isize;
  let mut cur_cycle = 0usize;
  let mut signal_strength_sum = 0isize;

  let screen_width = 40;
  let screen_height = 6;
  let sprite_width = 3;

  let mut screen = vec![vec![false; screen_width]; screen_height];
  let mut cursor_pos = (0usize, 0usize);

  for istn in istns {
    let cycle_duration = match istn {
      Istn::Noop => 1,
      Istn::Addx(_) => 2,
    };

    for _ in 0..cycle_duration {
      cur_cycle += 1;
      if cur_cycle == 20 || (cur_cycle >= 60 && (cur_cycle - 20) % 40 == 0) {
        let signal_str = cur_cycle as isize * x;
        signal_strength_sum += signal_str;
      }

      let sprite_middle_x = x;
      let pixel_is_lit = (sprite_middle_x - cursor_pos.0 as isize).abs() <= (sprite_width - 1) / 2;
      screen[cursor_pos.1][cursor_pos.0] = pixel_is_lit;

      cursor_pos.0 += 1;
      if cursor_pos.0 == screen_width {
        cursor_pos.0 = 0;
        cursor_pos.1 += 1;
      }
      if cursor_pos.1 == screen_height {
        cursor_pos.1 = 0;
      }
    }

    match istn {
      Istn::Noop => (),
      Istn::Addx(imm) => x += imm,
    }
  }

  println!("Part 1: {signal_strength_sum}");
  println!("Part 2:");
  print_screen(screen);
}
