import fs from 'fs/promises';

const maxJoltage = (bank: string[], desiredNumberOfBatteries: number = 2): number => {
    let result = '';
    let startIndex = 0;
    for (let i = 0; i < desiredNumberOfBatteries; i++) {
        let biggest = 0;
        // Find first biggest value
        for (let j = startIndex; j < bank.length; j++) {
            let val = parseInt(bank[i], 10);
            if (val > biggest && bank.length - i >= desiredNumberOfBatteries - j) {
                biggest = val;
                startIndex = i + 1;
            }
        }
        result += biggest.toString();
    }
    return parseInt(result, 10);
}

const solve = async () => {
    const buffer = await fs.readFile('input.txt');
    const banks = buffer.toString().split("\n").map(bank => bank.split('')).filter(bank => bank.length > 0);

    console.log(`Answer part one: ${banks.map(bank => maxJoltage(bank, 2)).reduce((a, b) => a + b, 0)}`);
    console.log(`Answer part one: ${banks.map(bank => maxJoltage(bank, 12)).reduce((a, b) => a + b, 0)}`);
};

solve();
