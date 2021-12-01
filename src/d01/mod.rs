use rust_2021::read_input;

const INPUT: &str = "src/d01/input";

pub fn run() {
  part1();
  part2();
}

fn part1() {
  let numbers = parse_input();

  let mut count = 0;

  for i in 0..numbers.len() - 1 {
    if numbers[i] < numbers[i+1] { count += 1}
  }
  println!("Part 1: {:?}", count)
}

fn part2() {
  let numbers = parse_input();

  let mut count = 0;

  for i in 0..numbers.len() - 3 {
    let a = numbers[i] + numbers[i + 1] + numbers[i+2];
    let b = numbers[i + 1] + numbers[i + 2] + numbers[i + 3];
    if a < b { count += 1}
  }
  println!("Part 2: {:?}", count)
}

fn parse_input() -> Vec<i32> {
  let input = read_input(INPUT);
  input.trim()
    .split("\n")
    .map(|n| n.parse::<i32>().unwrap())
    .collect()
}