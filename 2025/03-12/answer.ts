import fs from 'fs/promises';

const maxJoltage = (bank: string[], desiredNumberOfBatteries: number = 2): number => {
    let result = '';
    let startIndex = 0;

    // Construct the desired battery length
    for (let currentBatteryPosition = 0; currentBatteryPosition < desiredNumberOfBatteries; currentBatteryPosition++) {
        let biggestJoltage = 0;

        // Find the next biggest possible value
        for (let i = startIndex; i < bank.length; i++) {
            const joltage = parseInt(bank[i], 10);

            // Only mark as biggest if it's bigger and we can still construct the rest of the battery
            if (joltage > biggestJoltage && bank.length - i >= desiredNumberOfBatteries - currentBatteryPosition) {
                biggestJoltage = joltage;
                startIndex = i + 1;
            }
        }

        // Construct result as a string
        result += biggestJoltage.toString();
    }

    // Parse resulting string into a number so we can sum it later
    return parseInt(result, 10);
}

const solve = async () => {
    const buffer = await fs.readFile('input.txt');
    const banks = buffer.toString().split("\n").map(bank => bank.split('')).filter(bank => bank.length > 0);

    console.log(`Answer part one: ${banks.map(bank => maxJoltage(bank, 2)).reduce((a, b) => a + b, 0)}`);
    console.log(`Answer part one: ${banks.map(bank => maxJoltage(bank, 12)).reduce((a, b) => a + b, 0)}`);
};

solve();
