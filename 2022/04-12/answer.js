import { readFile } from 'fs/promises';

const doesPairOverlapFully = pair => (pair[0] >= pair[2] && pair[1] <= pair[3]) || (pair[2] >= pair[0] && pair[3] <= pair[1]);
const doesPairNotOverlap = pair => pair[1] < pair[2] || pair[3] < pair[0];

const answer = async () => {
    const input = (await readFile('input.txt'))
        .toString()
        .split('\n')
        .map(line => line.split(','))
        .map(pair => pair.map(range => range.split('-')))
        .map(pair => pair.flat())
        .map(pair => pair.map(val => parseInt(val, 10)));

    const overlapsFully = input.filter(doesPairOverlapFully);
    const doesNotOverlap = input.filter(doesPairNotOverlap);
    
    console.log(`Answer 1: ${overlapsFully.length}`);
    console.log(`Answer 2: ${input.length - doesNotOverlap.length}`);
};

answer();
