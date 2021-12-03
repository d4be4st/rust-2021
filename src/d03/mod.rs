use std::ops::Index;

pub fn run() {
  part2();
}

fn part1() {
  let mut rates = [(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0),(0, 0)];
  include_str!("input")
  .lines()
  .for_each(|line| {
    let chars = line.chars().enumerate();
    for (i, c) in chars {
      if c == '0' { rates[i].0 += 1 } else { rates[i].1 += 1 }
    }
  });

  let mut output0 = String::new();
  let mut output1 = String::new();
  
  for (zero, one) in rates {
    if zero > one {
      output0.push('0');
      output1.push('1');
    } else {
      output0.push('1');
      output1.push('0');
    }
  }

  println!("Part 1: {}", isize::from_str_radix(output0.as_str(), 2).unwrap() * isize::from_str_radix(output1.as_str(), 2).unwrap())
}

fn part2() {
  let mut oxygen_lines: Vec<&str> = include_str!("input").lines().collect();
  let mut co2_lines = oxygen_lines.clone();

  for i in 0..12 {
    if oxygen_lines.len() == 1 { break; }
    let (oxy_ones, oxy_zeroes): (Vec<&str>, Vec<&str>) = oxygen_lines.iter().partition(|&line| {
      line.chars().nth(i).unwrap() == '1'
    });

    oxygen_lines = if oxy_ones.len() >= oxy_zeroes.len() { oxy_ones } else { oxy_zeroes }
  };

  for i in 0..12 {
    if co2_lines.len() == 1 { break; }
    let (co2_zeroes, co2_ones): (Vec<&str>, Vec<&str>) = co2_lines.iter().partition(|&line| {
      line.chars().nth(i).unwrap() == '0'
    });

    co2_lines = if co2_zeroes.len() <= co2_ones.len() { co2_zeroes } else { co2_ones }
  };

  let oxy = isize::from_str_radix(oxygen_lines[0], 2).unwrap();
  let co2 = isize::from_str_radix(co2_lines[0], 2).unwrap();

  println!("Part 2: {}", oxy * co2);
}