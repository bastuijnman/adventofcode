import { readFile } from 'fs/promises';

const priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const split = r => [r.substring(0, r.length / 2), r.substring(r.length / 2)];
const compare = r => r[0].split('').filter(c => r[1].includes(c)).concat(r[1].split('').filter(c => r[0].includes(c)));
const unique = r => r.filter((c, i) => r.indexOf(c) === i);
const calculatePriority = r => priorities.indexOf(r[0]) + 1;
const sum = (cur, prev) => prev + cur;

const answer = async () => {
    const input = (await readFile('input.txt'))
        .toString()
        .split('\n');

    const prioritySum = input
        .map(split)
        .map(compare)
        .map(unique)
        .map(calculatePriority)
        .reduce(sum, 0);

    console.log(`Answer 1: ${prioritySum}`);
};

answer();