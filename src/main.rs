use std::borrow::Borrow;
use std::collections::HashMap;

mod helpers;

const LOWER_A: u8 = b'a';
const LOWER_Z: u8 = b'z';
const UPPER_A: u8 = b'A';
const UPPER_Z: u8 = b'Z';
// const LOWER_OFFSET: u8 = (1 - LOWER_A);
// const UPPER_OFFSET: u8 = (27 - UPPER_A);
const LOWER_OFFSET: u8 = (1 - LOWER_A as i8).abs() as u8;
const UPPER_OFFSET: u8 = (27 - UPPER_A as i8).abs() as u8;

fn main() {
    day3();
}

fn day3() {
    let input = helpers::file_reader::read_file_for_day("3");
    let line_endings = helpers::file_reader::get_line_endings(input.borrow());
    // let mut groups = input.split(line_endings).collect::<Vec<&str>>().chunks(3);
    // println!("Fasdad {:?}", groups.clone());
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
    let badge_prios: Vec<u8> = group_badges.iter().map(|c| return day3_get_priority(*c as u8)).collect();
    let second_total: i32 = badge_prios.iter().map(|v| *v as i32).sum();
    println!("Second Answer {:?} Contents {:?}", second_total, badge_prios);
}

fn day3_get_priority(c: u8) -> u8 {
    println!("a: {:?} z: {:?} lowoff: {:?} A: {:?} Z: {:?} upoff: {:?} c: {:?}", LOWER_A, LOWER_Z, LOWER_OFFSET, UPPER_A, UPPER_Z, UPPER_OFFSET, c);
    return if c >= LOWER_A && c <= LOWER_Z {
        println!("letter {:?} a {:?} <= c {:?} >= z {:?} : c {:?} - lowoff {:?} = {:?}", c as char, LOWER_A, c, LOWER_Z, c, LOWER_OFFSET, c - LOWER_OFFSET);
        c - LOWER_OFFSET
    } else if c >= UPPER_A && c <= UPPER_Z {
        println!("letter {:?} A {:?} <= c {:?} >= Z {:?} : c {:?} - upoff {:?} = {:?}", c as char, UPPER_A, c, UPPER_Z, c, UPPER_OFFSET, c - UPPER_OFFSET);
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
