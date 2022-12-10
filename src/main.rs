use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::{Add, Index};
use regex::Regex;use std::collections::VecDeque;

mod helpers;

fn main() {
    day5();
}

fn day5() {
    let input = helpers::file_reader::read_file_for_day("5");
    let line_endings = helpers::file_reader::get_line_endings(input.borrow());
    let section_sep = "".to_owned().add(line_endings).add(line_endings);
    let mut sections: Vec<Vec<&str>> = input.split(&section_sep).map(|x| x.split(line_endings).collect()).collect();

    let sec_len = sections[0].len();
    let stack_list: Vec<i32> = sections[0].split_off(sec_len - 1)[0].split("   ").map(|x| x.trim().parse::<i32>().unwrap()).collect();
    let mut stacks: HashMap<i32, VecDeque<String>> = HashMap::new();

    // Initialize our stacks as vectors which we will use as fifo through
    for stack in stack_list.iter() {
        stacks.insert(*stack, VecDeque::new());
    }

    println!("row {:?}", stacks);

    let cargo_regex = Regex::new(r"[\[ ]([A-Z ])[] ] ?").unwrap();
    for row in sections[0].iter() {
        // println!("row {:?}", row);
        let mut index = 0;
        let mut val: String;
        for cap in cargo_regex.captures_iter(row) {
            val = cap[1].borrow().trim().parse().unwrap();
            // println!("{:?} {:?}", cap, index);
            if !val.is_empty() {
                index += 1;
                // println!("Adding {:?} to {:?}", val, index);
                stacks.entry(index).and_modify(|s| {
                    s.push_back(val);
                });
            } else {
                // If we encountered an empty column, still need to count it
                index += 1;
                // println!("empty col {}", index);
            }
        }
    }
    println!("Stacks {:?}", stacks);

    let moves_regex = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let mut moves: Vec<Vec<i32>> = vec![];
    for row in sections[1].iter() {
        let mut val: String;
        for cap in moves_regex.captures_iter(row) {
            if !cap[1].is_empty() {
                moves.push(vec![cap[1].trim().parse::<i32>().unwrap(), cap[2].trim().parse::<i32>().unwrap(), cap[3].trim().parse::<i32>().unwrap()]);
            }
        }
    }
    println!("Moves {:?}", moves);

    for moveset in moves.iter() {
        // println!("Stacks B {:?}", stacks);
        for i in 0..moveset[0] {
            let mut temp: String = "".parse().unwrap();
            stacks.entry(moveset[1]).and_modify(|s| {
                temp = s.pop_front().unwrap();
                // println!("Grabbed {:?} from {:?}", temp, moveset[1]);
            });

            stacks.entry(moveset[2]).and_modify(|s| {
                if !temp.is_empty() {
                    s.insert(0, temp);
                }
            });
        }
        // println!("Stacks A {:?}", stacks);
    }

    println!("Stacks {:?}", stacks);

    let stack_len = stacks.keys().len() as i32;
    let mut first_ans: String = "".parse().unwrap();
    for i in 1..stack_len + 1 {
        first_ans = first_ans.add(&stacks.get_key_value(i.borrow()).unwrap().1[0]);
    }
    // println!("First Answer {}{}{}", stacks.get_key_value(&1).unwrap().1[0], stacks.get_key_value(&2).unwrap().1[0], stacks.get_key_value(&3).unwrap().1[0]);
    println!("First Answer {:?}", first_ans);

    // let stacks: Vec<Vec<&str>> =
    // let data: Vec<Vec<&str>> = input.split(line_endings).map(|d| d.split("  ").map(|x| x.trim()).collect()).collect();
    // println!("{:?}", sections);
    // for row in data {
    //
    // }
    // let pairs: Vec<Vec<Vec<i32>>> = input.split(line_endings).map(|p| p.split(",").map(|q| {
    //     q.split("-").map(|v| if !v.is_empty() { v.parse::<i32>().unwrap() } else { 0 }).collect()
    // }).collect()).collect();
}

fn day4() {
    let input = helpers::file_reader::read_file_for_day("4");
    let line_endings = helpers::file_reader::get_line_endings(input.borrow());
    let pairs: Vec<Vec<Vec<i32>>> = input.split(line_endings).map(|p| p.split(",").map(|q| {
        q.split("-").map(|v| if !v.is_empty() { v.parse::<i32>().unwrap() } else { 0 }).collect()
    }).collect()).collect();

    let mut num_fully_contained: i32 = 0;
    let mut num_groups_overlap: i32 = 0;
    for wg in pairs.iter() {
        if (wg.len() >= 2) {
            let mut assigns: HashMap<i32, i32> = HashMap::new();
            let mut overlaps: Vec<i32> = vec![];
            let left = wg.get(0).unwrap();
            let left_total = left[1] - left[0] + 1;
            for num in left[0]..left[1] + 1 {
                *assigns.entry(num).or_default() += 1;

                if assigns.get(&num).unwrap() >= &2 {
                    overlaps.push(num);
                }
            }
            let right = wg.get(1).unwrap();
            let right_total = right[1] - right[0] + 1;
            for num in right[0]..right[1] + 1 {
                *assigns.entry(num).or_default() += 1;

                if assigns.get(&num).unwrap() >= &2 {
                    overlaps.push(num);
                }
            }

            let num_overlaps: i32 = overlaps.len() as i32;
            // println!("left {:?} ltot {:?} right {:?} rtot {:?} otot {:?} overlaps {:?}", left, left_total, right, right_total, num_overlaps, overlaps);
            if left_total <= num_overlaps || right_total <= num_overlaps {
                num_fully_contained += 1;
                println!("Full Contain Detected: left {:?} ltot {:?} right {:?} rtot {:?} otot {:?} overlaps {:?}", left, left_total, right, right_total, num_overlaps, overlaps);
            }

            if num_overlaps >= 1 {
                println!("{:?}", num_overlaps);
                num_groups_overlap += 1;
            }
        }
    }
    println!("First Answer: {:?}", num_fully_contained);
    println!("Second Answer: {:?}", num_groups_overlap);
}

fn day3() {
    let input = helpers::file_reader::read_file_for_day("3");
    let line_endings = helpers::file_reader::get_line_endings(input.borrow());
    let indiv_rucksack: Vec<&str> = input.split(line_endings).collect();
    let group_badges: Vec<char> = indiv_rucksack.chunks(3).map(|group| {
        let mut seen_group_chars: HashMap<u8, i32> = HashMap::new();
        let mut pos_badges: Vec<&u8> = vec![];

        for inv in group {
            let mut seen_chars: Vec<u8> = vec![];
            for byt in inv.as_bytes().iter() {
                if !seen_chars.contains(byt) {
                    seen_chars.push(*byt);
                    *seen_group_chars.entry(*byt).or_default() += 1;

                    if *seen_group_chars.get(byt).unwrap() == 3 {
                        pos_badges.push(byt);
                    }
                }
            }
        }

        if pos_badges.len() >= 1 {
            *pos_badges[0] as char
        } else {
            '-'
        }
    }).collect();

    let mut rucksack_contents: Vec<u8> = input.split(line_endings).map(|s| {
        let compartments = s.split_at(s.len() / 2);
        let mut priority = 0;
        let mut seen_chars: Vec<u8> = vec![];
        for byt in compartments.0.as_bytes() {
            if compartments.1.as_bytes().contains(byt) && !seen_chars.contains(byt) {
                seen_chars.push(*byt);
                priority += day3_get_priority(byt.clone());
            }
        }
        return priority;
    }).collect();

    let total: i32 = rucksack_contents.iter().map(|v| *v as i32).sum();
    println!("First Answer {:?} Contents {:?}", total, rucksack_contents);
    let badge_prios: Vec<u8> = group_badges.iter().map(|c| day3_get_priority(*c as u8)).collect();
    let second_total: i32 = badge_prios.iter().map(|v| *v as i32).sum();
    println!("Second Answer {:?} Contents {:?}", second_total, badge_prios);
}

const LOWER_A: u8 = b'a';
const LOWER_Z: u8 = b'z';
const UPPER_A: u8 = b'A';
const UPPER_Z: u8 = b'Z';
const LOWER_OFFSET: u8 = (1 - LOWER_A as i8).abs() as u8;
const UPPER_OFFSET: u8 = (27 - UPPER_A as i8).abs() as u8;

fn day3_get_priority(c: u8) -> u8 {
    // println!("a: {:?} z: {:?} lowoff: {:?} A: {:?} Z: {:?} upoff: {:?} c: {:?}", LOWER_A, LOWER_Z, LOWER_OFFSET, UPPER_A, UPPER_Z, UPPER_OFFSET, c);
    return if c >= LOWER_A && c <= LOWER_Z {
        // println!("letter {:?} a {:?} <= c {:?} >= z {:?} : c {:?} - lowoff {:?} = {:?}", c as char, LOWER_A, c, LOWER_Z, c, LOWER_OFFSET, c - LOWER_OFFSET);
        c - LOWER_OFFSET
    } else if c >= UPPER_A && c <= UPPER_Z {
        // println!("letter {:?} A {:?} <= c {:?} >= Z {:?} : c {:?} - upoff {:?} = {:?}", c as char, UPPER_A, c, UPPER_Z, c, UPPER_OFFSET, c - UPPER_OFFSET);
        c - UPPER_OFFSET
    } else {
        println!("oof {}", c);
        0
    };
}

fn day1() {
    let input = helpers::file_reader::read_file_for_day("1");
    let block_endings = helpers::file_reader::get_block_endings(input.borrow());
    let line_endings = helpers::file_reader::get_line_endings(input.borrow());
    let mut elf_inv: Vec<i32> = input.split(block_endings).map(|s| {
        let inv = s.trim().split(line_endings);
        let mut accum = 0;
        for item in inv {
            accum += item.parse::<i32>().unwrap();
        }
        return accum;
    }).collect();
    elf_inv.sort_by(|a, b| b.cmp(a));
    println!("First Answer {:?}", elf_inv[0]);
    println!("Second Answer {:?}", elf_inv[0] + elf_inv[1] + elf_inv[2]);
}
