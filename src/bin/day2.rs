use adventofcode_2023::runner;
use hashbrown::HashMap;
use itertools::Itertools;

const MAX_RED : u32 = 12;
const MAX_GREEN : u32 = 13;
const MAX_BLUE : u32 = 14;
type Colors = (u32, u32, u32);

fn parse_input(input: &str) -> HashMap<usize, Vec<Colors>> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap();
            let id = sscanf::sscanf!(left, "Game {usize}").unwrap();
            let game = parse_game(right);
            (id, game)
        }).collect::<HashMap<usize, Vec<Colors>>>()
}

fn parse_game(game: &str) -> Vec<Colors> {
    game.trim()
        .split(';')
        .map(|set| {
            let (mut r, mut g, mut b) = (0,0,0);
            set.trim().split(',').for_each(|color| {
                let (v, c) = sscanf::sscanf!(color.trim(), "{u32} {str}").unwrap();
                match c {
                    "red" => r += v,
                    "green" => g += v,
                    "blue" => b += v,
                    _ => panic!("Invalid color")
                }
            });
            (r, g, b)
        })
            .collect_vec()
}

fn part1(input: &str) {
    let games = parse_input(input);
    let mut impossible_ids = Vec::new();

    'outer: for (id,  game) in games.iter() {
        for (r, g, b) in game.iter() {
            if *r > MAX_RED || *g > MAX_GREEN || *b > MAX_BLUE {
                impossible_ids.push(*id);
                continue 'outer;
            }
        }
    }

    let games_sums = games.keys().sum::<usize>();
    println!("Day 2 Part 1: {}", games_sums - impossible_ids.iter().sum::<usize>());

}


fn part2(input: &str) {
    let games = parse_input(input);
    let game_powers =  games.iter().map(|(_, game)| {
        let max_red = game.iter().map(|(red, _, _)| red).max().unwrap();
        let max_green= game.iter().map(|(_, green, _)| green).max().unwrap();
        let max_blue= game.iter().map(|(_, _, blue)| blue).max().unwrap();
        max_red * max_green * max_blue
    })
        .sum::<u32>();

    println!("Day 2 Part 2: {}", game_powers);
}

fn main() {
    runner(part1);
    runner(part2);
}
