import {readFileForDay, splitBlocks, splitLines} from './helper.js';
let input = await readFileForDay(2);

const mappings = {
    Rock: 'A',
    Paper: 'B',
    Scissors: 'C',
    PlayerRock: 'X',
    PlayerPaper: 'Y',
    PlayerScissors: 'Z',
    Lose: 'X',
    Draw: 'Y',
    Win: 'Z',
};

function doFirstPuz() {
    const scores = {
        A: 1,
        B: 2,
        C: 3,
        X: 1,
        Y: 2,
        Z: 3,
        win: 6,
        draw: 3,
        loss: 0
    }

    let totalScore = 0;
    let result = splitLines(input).map((row) => {
        return row.split(' ');
    }).map((round) => {
        // Add score for what was chosen (rock/paper/scissors)
        totalScore += scores[round[1]];

        // Determine points for winner
        let playerWins = false;
        let draw = scores[round[0]] === scores[round[1]];
        if (!draw) {
            switch (round[0]) {
                case mappings.Rock:
                    playerWins = round[1] === mappings.PlayerPaper;
                    break;
                case mappings.Paper:
                    playerWins = round[1] === mappings.PlayerScissors;
                    break;
                case mappings.Scissors:
                    playerWins = round[1] === mappings.PlayerRock;
                    break;
                default:
                    console.error(`Unknown input "${round[0]}"`);
                    break;
            }
        }

        if (playerWins) {
            totalScore += scores.win;
        } else if (draw) {
            totalScore += scores.draw;
        } else {
            totalScore += scores.loss;
        }
    });
    console.log(totalScore);
}

doFirstPuz(input);

function matchFix() {
    const scores = {
        A: 1,
        B: 2,
        C: 3,
        X: 0, // Lose
        Y: 3, // Draw
        Z: 6, // Win
    }

    let totalScore = 0;
    let result = splitLines(input).map((row) => {
        return row.split(' ');
    }).map((round) => {
        switch (round[1]) {
            case mappings.Lose:
                if (round[0] === mappings.Rock) {
                    totalScore += scores[mappings.Scissors];
                } else if (round[0] === mappings.Paper) {
                    totalScore += scores[mappings.Rock];
                } else if (round[0] === mappings.Scissors) {
                    totalScore += scores[mappings.Paper];
                }
                break;
            case mappings.Draw:
                totalScore += scores[round[0]];
                break;
            case mappings.Win:
                if (round[0] === mappings.Rock) {
                    totalScore += scores[mappings.Paper];
                } else if (round[0] === mappings.Paper) {
                    totalScore += scores[mappings.Scissors];
                } else if (round[0] === mappings.Scissors) {
                    totalScore += scores[mappings.Rock];
                }
                break;
            default:
                break;
        }
        // Add score for result (win/lose/draw)
        totalScore += scores[round[1]];
    });
    console.log(totalScore);
}

matchFix(input);
