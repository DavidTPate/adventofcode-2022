import {readFileForDay, splitBlocks, splitLines} from './helper.js';

try {
    let input = await readFileForDay(1);

    let elves = splitBlocks(input).map((inv) => {
        return splitLines(inv)
    }).map((inv,) => {
        return inv.reduce((agg, itemCals) => {
            return agg += itemCals * 1;
        }, 0);
    }).sort((a, b) => {
        if (a < b) {
            return 1;
        } else if (a > b) {
            return -1;
        } else {
            return 0;
        }
    });
    // first puzzle
    console.log(elves[0])
    // second
    console.log(elves[0] + elves[1] + elves[2])
} catch (err) {
    console.error(err.message);
}