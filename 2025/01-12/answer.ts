import { promises as fs } from "fs";

// Make left rotation a negative calc and right rotation a positive one
const modifier = { L: -1, R: 1 };

// Make modulo work nicely with negative numbers.
const mod = (input: number, mod: number) => ((input % mod) + mod) % mod;

const main = async () => {
    const buffer = await fs.readFile('input.txt');
    const data = buffer.toString().matchAll(/(L|R)(\d+)/g);

    let dial: number = 50;
    let passwordV1: number = 0;
    let passwordV2: number = 0;
    for (const [, direction, turnsData] of data) {

        // Calculate the full rotations already made and how much we have left to rotate afterwards
        const turns = parseInt(turnsData, 10);
        const rotations = Math.floor(turns / 100);
        const remainder = turns - (rotations * 100);

        const updatedDial = dial + (modifier[direction] * remainder);

        // If we arrive at exactly 0 we increment the password
        if (mod(updatedDial, 100) === 0) {
            passwordV1 += 1;
        }

        // Add all full rotations and calculate if we rotate still with the remainder
        passwordV2 += rotations;
        if (dial !== 0 && updatedDial <= 0 || updatedDial > 99) {
            passwordV2 += 1;
        }

        // Update dial to new position
        dial = mod(updatedDial, 100);
    }

    console.log(`Answer part one: ${passwordV1}`);
    console.log(`Answer part two: ${passwordV2}`);
};

main();
