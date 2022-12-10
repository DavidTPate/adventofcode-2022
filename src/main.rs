use std::borrow::Borrow;

mod helpers;

fn main() {
    day2();
}

fn day2() {

}

fn day1() {
    let input = helpers::file_reader::read_file_for_day("1");
    let block_endings = helpers::file_reader::get_block_endings(input.borrow());
    let line_endings = helpers::file_reader::get_line_endings(input.borrow());
    let mut elf_inv: Vec<u32> = input.split(block_endings).map(|s| {
        let inv = s.trim().split(line_endings);
        let mut accum = 0;
        for item in inv {
            accum += item.parse::<u32>().unwrap();
        }
        return accum;
    }).collect();
    elf_inv.sort_by(|a, b| b.cmp(a));
    println!("First Answer {:?}", elf_inv[0]);
    println!("Second Answer {:?}", elf_inv[0] + elf_inv[1] + elf_inv[2]);
}
