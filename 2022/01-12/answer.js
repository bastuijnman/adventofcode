import { readFile } from 'fs/promises';

const answer = async () => {

    const input = await readFile('input.txt');
    const elves = input.toString().split('\n\n');

    const calories = elves
        .map(elf => elf.split("\n").reduce((prev, cur) =>  prev + parseInt(cur, 10), 0))
        .sort((a, b) => b-a);
    
    console.log(`Answer 1: ${calories[0]}`);
};

answer();