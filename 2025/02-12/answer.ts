import fs from 'fs/promises';

// Inclusive range function
const range = (start: number, end: number) => [...Array((end + 1) - start).keys()].map(i => i + start);

// Check if an ID is invalid at the fold of the number
const isInvalidIdByFold = (id: number): boolean => {
    const check = id.toString();
    const middle = check.length / 2;
    return check.slice(0, middle) === check.slice(middle)
};

// Check if an ID is invalid by checking for patterns
const isInvalidIdByPattern = (id: number): boolean => {
    const check = id.toString();

    // Loop through possible patterns, ignore ids with just one digit
    for (let i = 0; i < Math.floor(check.length / 2); i++) {
        const pattern = check.slice(0, i + 1);

        // Make sure the extended pattern is the same length as the check value
        // and validate that it forms the same string.
        if (check.length % pattern.length === 0 && pattern.padEnd(check.length, pattern) === check) {
            return true;
        }
    }
    return false;
}

const sum = (a: number, b: number): number => a + b;

const solve = async () => {
    const buffer = await fs.readFile('input.txt');
    const ranges = buffer.toString().matchAll(/(\d+)\-(\d+)/g);

    // Construct ID's from the given ranges
    const ids = Array.from(ranges)
        .map((r: string[]) => [parseInt(r[1], 10), parseInt(r[2], 10)])
        .map((r: number[]) => range(r[0], r[1]))
        .flat();

    console.log(`Answer part one: ${ids.filter(isInvalidIdByFold).reduce(sum, 0)}`);
    console.log(`Answer part two: ${ids.filter(isInvalidIdByPattern).reduce(sum, 0)}`);
}

solve();
