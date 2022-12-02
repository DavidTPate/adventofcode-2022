import {readFile} from 'node:fs/promises';
const useTestInput = false;
export async function readFileForDay(day) {
    let filePath = new URL(`./input/day${day}.txt`, import.meta.url);
    if (useTestInput) {
        filePath = new URL(`./input/day${day}-test.txt`, import.meta.url);
    }

    try {
        let input = await readFile(filePath, {encoding: 'utf8'});
        return input;
    } catch (err) {
        console.error(err.message);
        return Promise.reject(err);
    }
}

export function splitBlocks(input) {
    let windowsEndings = input.indexOf("\r") >= 0;
    return input.split(windowsEndings ? "\r\n\r" : "\n\n");
}

export function splitLines(input) {
    let windowsEndings = input.indexOf("\r") >= 0;
    return input.split(windowsEndings ? "\r\n" : "\n");
}