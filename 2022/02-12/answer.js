import { readFile } from 'fs/promises';

const scoreMatrix = { 'A': 'ZXY', 'B': 'XYZ', 'C': 'YZX' };
const score = game => 'XYZ'.indexOf(game[2])+1;
const outcome = game => scoreMatrix[game[0]].indexOf(game[2]) * 3;
const sanitize = game => `${game[0]} ${scoreMatrix[game[0]]['XYZ'.indexOf(game[2])]}`;
const cummulative = (prev, cur) => prev + cur;

const answer = async () => {
    const input = (await readFile('input.txt'))
        .toString()
        .split('\n');
    
    const totals = input
        .map(game => outcome(game) + score(game));

    const totalsFixed = input
        .map(game => sanitize(game))
        .map(game => outcome(game) + score(game));

    console.log(`Answer 1: ${totals.reduce(cummulative)}`);
    console.log(`Answer 1: ${totalsFixed.reduce(cummulative)}`);
}

answer();