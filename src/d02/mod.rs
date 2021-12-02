pub fn run() {
  part2()
}

fn _part1() {
  let (poistion, depth) = 
    include_str!("input")
    .lines()
    .fold((0, 0), |mut acc, line| -> (i32, i32) {
      let split: Vec<&str> = line.split(" ").collect();
      let number: i32 = split[1].parse().unwrap();
      match split[0] {
        "forward" => acc.0 += number,
        "down" => acc.1 += number,
        "up" => acc.1 -= number,
        _ => {}
      };
      acc
    });

    println!("Part 1: {}", poistion * depth);
}

fn part2() {
  let (poistion, depth, _aim) = 
    include_str!("input")
    .lines()
    .fold((0, 0, 0), |mut acc, line| -> (i32, i32, i32) {
      let split: Vec<&str> = line.split(" ").collect();
      let number: i32 = split[1].parse().unwrap();
      match split[0] {
        "forward" => {
          acc.0 += number;
          acc.1 += number * acc.2;
        },
        "down" => acc.2 += number,
        "up" => acc.2 -= number,
        _ => {}
      };
      acc
    });

    println!("Part 2: {}", poistion * depth);
}