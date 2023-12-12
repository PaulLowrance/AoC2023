use std::cell::RefCell;

use adventofcode_2023::runner;
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

struct Game {
    id: u32,
    winning_numbers: Vec<u32>,
    player_numbers: Vec<u32>,
    point_total: u32,
}

fn parse_row_to_game(input: &str) -> Game {
    //println!("{}", input);

    let (game_tag, all_numbers) = input.split_once(':').unwrap();
    //let id = sscanf::sscanf!(game_tag, "Card {u32}").unwrap();
    let id = game_tag
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let (winning_num_string, player_num_string) = all_numbers.split_once('|').unwrap();
    // println!("--{}--", winning_num_string.trim());
    // println!("--{}--", player_num_string.trim());
    let winning_numbers = winning_num_string
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect_vec();
    let player_numbers = player_num_string
        .split_whitespace()
        .map(|num| num.parse::<u32>().unwrap())
        .collect_vec();

    Game {
        id,
        winning_numbers,
        player_numbers,
        point_total: 0,
    }
}

fn get_intersection_count(a: &Vec<u32>, b: &Vec<u32>) -> u32 {
    let a_set: HashSet<u32> = a.iter().cloned().collect();
    let b_set: HashSet<u32> = b.iter().cloned().collect();
    a_set.intersection(&b_set).count() as u32
}

fn part1(input: &str) {
    /*Steps to solve:
     * 1. parse rows into a data structure that  has an Id, vec of winning numbers, vec of player
     *    numbers, and a point total
     * 2. Iterate over these structures
     * 3. Iterate over the winning numbers and see if any exist
     * 4. Do the multiplication
     */

    let mut game_cards = input.lines().map(parse_row_to_game).collect_vec();

    for game_card in game_cards.iter_mut() {
        let mut point_count = 0;
        for winning_number in game_card.winning_numbers.iter() {
            if game_card.player_numbers.contains(winning_number) {
                point_count += 1;
            }
        }
        // println!("card id: {}, point count: {}", game_card.id, point_count);
        for _p in 1..=point_count {
            if game_card.point_total < 2 {
                game_card.point_total += 1;
            } else {
                game_card.point_total *= 2;
            }
        }
        // println!("card id: {}, point total: {}", game_card.id, game_card.point_total);
    }

    let point_total_sum = game_cards.iter().map(|game| game.point_total).sum::<u32>();

    println!("Day 4 Part 1: {}", point_total_sum);
}

fn parse_text(txt: &str) -> Vec<u32> {
    let re = regex::Regex::new(r"\b\d{1,2}\b").unwrap();
    re.captures_iter(txt)
        .map(|c| c.get(0).unwrap().as_str().parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn parse_input(input: &str) -> Vec<(Vec<u32>, Vec<u32>)> {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(':').unwrap().1.split_once('|').unwrap();
            (parse_text(left), parse_text(right))
        })
        .collect_vec()
}

fn part2(input: &str) {
    let game_cards: HashMap<usize, (Vec<u32>, Vec<u32>, RefCell<usize>)> = parse_input(input)
        .iter()
        .enumerate()
        .map(|(id, (l, r))| (id + 1, (l.clone(), r.clone(), RefCell::new(1))))
        .collect();

    for (id, (l, r, count)) in game_cards.iter().sorted() {
        let match_count = get_intersection_count(l, r) as usize;

        (id + 1..=id + match_count)
            .for_each(|index| *game_cards.get(&index).unwrap().2.borrow_mut() += *count.borrow());
    }

    let total_cards = game_cards
        .values()
        .map(|(_, _, count)| *count.borrow())
        .sum::<usize>();

    println!("Day 4 Part 2: {}", total_cards);
}

fn main() {
    runner(part1);
    runner(part2);
}
