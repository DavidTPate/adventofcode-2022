mod helpers;

fn main() {
    let useTestInput = false;
    let input = helpers::file_reader::read_file_in_cwd(if useTestInput { "../input/day1-test.txt" } else { "../input/day1.txt" });
    let windowsEndings = input.contains("\r");
    let blockSeparator = if windowsEndings { "\r\n\r" } else { "\n\n" };
    let lineSeparator = if windowsEndings { "\r\n" } else { "\n" };
    let elves: Vec<&str> = input.split(blockSeparator).map(|s| s.trim()).filter(|s| !s.is_empty()).collect();
    let elvesDetailInv: Vec<Vec<i32>> = elves.iter().map(|inv| inv.split(lineSeparator).map(|x| x.parse().unwrap()).collect()).collect();
    let mut elvesTotalInv: Vec<i32> = elvesDetailInv.iter().map(|x| x.into_iter().sum()).collect();
    elvesTotalInv.sort_by(|a, b| b.cmp(a));

    if (useTestInput) {
        assert_eq!(24000, elvesTotalInv[0]);
        assert_eq!(11000, elvesTotalInv[1]);
        assert_eq!(10000, elvesTotalInv[2]);
        assert_eq!(6000, elvesTotalInv[3]);
        assert_eq!(4000, elvesTotalInv[4]);
    } else {
        println!("First Answer {:?}", elvesTotalInv[0]);
        println!("Second Answer {:?}", elvesTotalInv[0] + elvesTotalInv[1] + elvesTotalInv[2]);

    }
}