use adventofcode_2023::runner;
use itertools::Itertools;
use hashbrown::HashSet;


type Grid = Vec<Vec<char>>;
type Pos = (usize, usize);
type PartNumber = Vec<(Pos, u32)>;

fn find_part_numbers(input: &str) -> Vec<PartNumber>{
    let grid: Grid = input.lines()
        .map(|line| line.chars()
            .collect())
        .collect();

    let mut part_numbers: Vec<PartNumber> = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        let mut window: PartNumber = Vec::new();
        for (x, c) in row.iter().enumerate() {
            if c.is_ascii_digit() {
                //println!("part number to digit: {}", c);
                window.push(((x, y), c.to_digit(10).unwrap()));
            } else if !window.is_empty() {
                part_numbers.push(window.clone());
                window.clear();
            }
        }
        if !window.is_empty() {
            part_numbers.push(window.clone());
        }
    }
    return part_numbers;
}

fn valid_neighbors(pos: Pos, grid: &Grid) -> Vec<Pos> {
    let y_max = grid.len();
    let x_max = grid[0].len();

    //I totally lifted this from another solution. 

    (-1..=1).cartesian_product(-1..=1)
        .map(|(dx, dy)| (pos.0 as isize + dx, pos.1 as isize + dy))
            .filter_map(|(nx, ny)| {
                if nx < 0 || ny < 0 || nx >= x_max as isize || ny >= y_max as isize {
                    return None;
                }
                return Some((nx as usize, ny as usize));
            }).collect_vec()
}

fn part_is_adjacent(part: &PartNumber, grid: &Grid) -> bool {
    return part.iter().flat_map(|((x, y), _)| {
        valid_neighbors((*x, *y), grid).into_iter().map(|(nx, ny)| {
            if grid[ny][nx].is_digit(10) || grid[ny][nx] == '.' {
                return false;
            }
            return true;
        })
    }).any(|x| x);
}


fn part1(input: &str) {
    let grid: Grid = input.lines()
        .map(|line| line.chars()
            .collect())
        .collect();

    let part_numbers = find_part_numbers(input)
        .iter()
        .filter(|&part| part_is_adjacent(part, &grid))
        .map(|part| part.iter().map(|(_, num)| *num).collect_vec())
        .map(|v| v.iter().fold(0, |acc, x| acc * 10 + x))
        .collect_vec();
        

    println!("Day 3 Part 1: {}", part_numbers.iter().sum::<u32>());
}

fn part2(input: &str) {
    /* Steps to solve:
    * 1. Find all the gear symbols
    * 2. Find all the numbers adjacent to the gear symbols
    * 3. multiply the adjacent numbers together and store in Vec
    * 4. sum the Vec
    * */

    //Get the grid from the input
    let grid: Grid = input.lines()
        .map(|line| line.chars()
            .collect())
        .collect();

    //get all the gear locations by
    //iterating through the rows
    //then flattening them and mapping them to get jsut the cells that have a gear
    let gears = grid.iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x,y,c)))
        .filter(|(_,_,c)| **c == '*')
        .map(|(x, y, _)| (x,y))
        .collect_vec();

    //get the part numbers & their locations
    let part_numbers = find_part_numbers(input);

    let mut gear_ratios: Vec<u32> = Vec::new();

    //iterate through the gears
    //find the valid neighbors for the gear
    //if any of the valid neighbors are numbers then check that position in the part numbers
    //if that position matches a part, then add it to the gear_numbers Vector
    //if there are two numbers in the gear_numbers Vector then multiply them together and add to gear_ratios
    for pos in gears {
        let mut gear_numbers: HashSet<PartNumber> = HashSet::new();
        for (x, y) in valid_neighbors(pos, &grid) {
            if grid[y][x].is_ascii_digit() {
                for part in part_numbers.iter() {
                    if part.iter().any(|((px, py), _)| *px == x && *py == y) {
                        gear_numbers.insert(part.clone());
                    }
                }
            }
        }
        if gear_numbers.len() == 2 {
            let gear_ratio = gear_numbers
                .iter()
                .map(|part| part.iter().map(|(_, num)| *num).collect_vec())
                .map(|v| v.iter().fold(0, |acc, x| acc * 10 + x))
                .product::<u32>();
            gear_ratios.push(gear_ratio);
        }
    }


    println!("Day 3 Part 2: {}", gear_ratios.iter().sum::<u32>());
}

fn main() {
    runner(part1);
    runner(part2);
}
