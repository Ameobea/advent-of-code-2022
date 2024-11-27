const INPUT: &'static str = include_str!("../inputs/day2.txt");

const ROCK: [char; 2] = ['A', 'X'];
const PAPER: [char; 2] = ['B', 'Y'];
const SCISSORS: [char; 2] = ['C', 'Z'];

#[derive(PartialEq, Clone, Copy, Debug)]
enum DesiredOutcome {
  Lose,
  Draw,
  Win,
}

impl DesiredOutcome {
  fn from_move(m: Move) -> DesiredOutcome {
    match m {
      Move::Rock => DesiredOutcome::Lose,
      Move::Paper => DesiredOutcome::Draw,
      Move::Scissors => DesiredOutcome::Win,
    }
  }
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Move {
  Rock,
  Paper,
  Scissors,
}

impl Move {
  fn score(&self) -> usize {
    match self {
      Move::Rock => 1,
      Move::Paper => 2,
      Move::Scissors => 3,
    }
  }

  fn from_char(c: char) -> Move {
    match c {
      c if c == ROCK[0] || c == ROCK[1] => Move::Rock,
      c if c == PAPER[0] || c == PAPER[1] => Move::Paper,
      c if c == SCISSORS[0] || c == SCISSORS[1] => Move::Scissors,
      _ => panic!("Invalid move: {}", c),
    }
  }
}

const POINTS_FOR_WIN: usize = 6;
const POINTS_FOR_DRAW: usize = 3;

fn get_move_for_outcome(their_move: Move, desired_outcome: DesiredOutcome) -> Move {
  match desired_outcome {
    DesiredOutcome::Lose => match their_move {
      Move::Rock => Move::Scissors,
      Move::Paper => Move::Rock,
      Move::Scissors => Move::Paper,
    },
    DesiredOutcome::Draw => their_move,
    DesiredOutcome::Win => match their_move {
      Move::Rock => Move::Paper,
      Move::Paper => Move::Scissors,
      Move::Scissors => Move::Rock,
    },
  }
}

fn play_round(m0: Move, m1: Move) -> (usize, usize) {
  match (m0, m1) {
    _ if m0 == m1 => (m0.score() + POINTS_FOR_DRAW, m1.score() + POINTS_FOR_DRAW),
    (Move::Rock, Move::Scissors) => (m0.score() + POINTS_FOR_WIN, m1.score()),
    (Move::Rock, Move::Paper) => (m0.score(), m1.score() + POINTS_FOR_WIN),
    (Move::Paper, Move::Rock) => (m0.score() + POINTS_FOR_WIN, m1.score()),
    (Move::Paper, Move::Scissors) => (m0.score(), m1.score() + POINTS_FOR_WIN),
    (Move::Scissors, Move::Paper) => (m0.score() + POINTS_FOR_WIN, m1.score()),
    (Move::Scissors, Move::Rock) => (m0.score(), m1.score() + POINTS_FOR_WIN),
    _ => unreachable!(),
  }
}

fn parse_input() -> Vec<(Move, Move)> {
  let mut moves = Vec::new();

  for line in INPUT.lines() {
    let mut chars = line.chars();
    let m0 = chars.next().unwrap();
    assert_eq!(chars.next().unwrap(), ' ');
    let m1 = chars.next().unwrap();
    assert_eq!(chars.next(), None);

    moves.push((Move::from_char(m0), Move::from_char(m1)));
  }

  moves
}

pub fn solve() {
  let moves = parse_input();
  let mut your_points = 0usize;
  for &round in &moves {
    let (_their_score, your_score) = play_round(round.0, round.1);
    your_points += your_score;
  }

  println!("Part 1: {}", your_points);

  // p2
  let mut your_points = 0usize;
  for round in moves {
    let their_move = round.0;
    let desired_outcome = DesiredOutcome::from_move(round.1);
    let your_move = get_move_for_outcome(their_move, desired_outcome);
    let (_their_score, your_score) = play_round(their_move, your_move);
    your_points += your_score;
  }

  println!("Part 2: {}", your_points);
}
