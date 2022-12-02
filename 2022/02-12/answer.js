import { readFile } from 'fs/promises';

const scoreMatrix = {
    'A': ['X', 'Y'],
    'B': ['Y', 'Z'],
    'C': ['Z', 'X']
};

const score = game => 'XYZ'.indexOf(game[2])+1;
const outcome = (game) => (scoreMatrix[game[0]].indexOf(game[2]) + 1) * 3;

const answer = async () => {
    const input = await readFile('input.txt');

    const totals = input
        .toString()
        .split('\n')
        .map(game => outcome(game) + score(game));

    console.log(`Answer 1: ${totals.reduce((prev, cur) => prev + cur)}`);
}

answer();