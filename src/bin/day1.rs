use adventofcode_2023::runner;

fn parse_input(input: &str, replace: bool) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            if replace {
                line.to_string()
                    .replace("one", "o1e")
                    .replace("two", "t2w")
                    .replace("three", "th3")
                    .replace("four", "f4u")
                    .replace("five", "f5e")
                    .replace("six", "s6x")
                    .replace("seven", "se7en")
                    .replace("eight", "e8t")
                    .replace("nine", "n9ne")
            } else {
                line.to_string()
            }
        })
        .map(|line| {
            line.chars()
            .filter_map(|c| c.to_digit(10))
                .collect::<Vec<u32>>()
        })
        .map(|vec| 10 * vec.first().unwrap() + vec.last().unwrap())
        .sum()
}

fn part1(input: &str) {
    println!("Day 1 Part 1: {}", parse_input(input, false));
}

fn part2(input: &str) {
    println!("Day 1 Part 2: {}", parse_input(input, true));
}

fn main() {
    runner(part1);
    runner(part2);
}
