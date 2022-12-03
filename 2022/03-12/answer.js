import { readFile } from 'fs/promises';

const priorities = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

const split = r => [r.substring(0, r.length / 2), r.substring(r.length / 2)];
const compare = r => r[0].split('').filter(c => r.filter(s => s.includes(c)).length === r.length);
const calculatePriority = r => priorities.indexOf(r[0]) + 1;
const sum = (cur, prev) => prev + cur;

const answer = async () => {
    const input = (await readFile('input.txt'))
        .toString()
        .split('\n');

    const prioritySum = input
        .map(split)
        .map(compare)
        .map(calculatePriority)
        .reduce(sum, 0);

    const groupInput = [].concat(input);
    const groups = [];
    while (groupInput.length) { groups.push(groupInput.splice(0, 3)); };

    const groupPrioritySum = groups
        .map(compare)
        .map(calculatePriority)
        .reduce(sum, 0);

    console.log(`Answer 1: ${prioritySum}`);
    console.log(`Answer 2: ${groupPrioritySum}`);
};

answer();