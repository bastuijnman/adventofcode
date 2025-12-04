import fs from 'fs/promises';

// This lovely piece of code gives you the valid neighbouring indices of a grid flattened
// to a single line.
const resolveNeighbours = (index: number, width: number, height: number): number[] => {
    const maxIndex = (width * height) - 1;
    return [
        // Top Left
        (index - width - 1 >= 0) && ((index - width) % width !== 0) ? index - width - 1 : null,

        // Top
        (index - width >= 0) ? index - width : null,

        // Top Right
        (index - width + 1 >= 0) && ((index - width + 1) % width !== 0) ? index - width + 1 : null,

        // Left
        (index - 1 >= 0) && (index % width !== 0) ? index - 1 : null,

        // Right
        (index + 1 <= maxIndex) && ((index + 1) % width !== 0) ? index + 1 : null,

        // Bottom Left
        (index + width - 1 <= maxIndex) && ((index + width) % width !== 0) ? index + width - 1 : null,

        // Bottom 
        (index + width <= maxIndex) ? index + width : null,

        // Bottom Right
        (index + width + 1 <= maxIndex) && ((index + width + 1) % width !== 0) ? index + width + 1 : null,

    ].filter(neighbour => neighbour !== null);
};

const solve = async () => {
    const buffer = await fs.readFile('input.txt');
    const input = buffer.toString();

    const width = input.indexOf("\n"); // First newline index equals the width of the grid
    const height = input.replaceAll("\n", "").length / width; // Number of newlines equal the height of the grid
    const grid = input.replaceAll("\n", "").split(''); // Flatten grid into single string

    // Check which rolls are accessible on the current grid
    const getAccessibleRolls = () => grid.map((value, i) => {
        // We do not care about anything other than rolls.
        if (value !== '@') {
            return null;
        }

        // Map into an array of the amount of roll neighbours and our current index
        return [
            resolveNeighbours(i, width, height)

                // We only care abount neighbouring rolls
                .filter(neighbourIndex => grid[neighbourIndex] === '@')
                .length,
            i
        ];
    }).filter(val => val !== null).filter(val => val[0] < 4);

    console.log(`Answer part one: ${getAccessibleRolls().length}`);

    // Here we keep modifying the map after taking the accessible roles and adding to the total
    // until no more rolls can be retrieved.
    let rolls = getAccessibleRolls();
    let total = 0;
    while (rolls.length > 0) {
        total += rolls.length;

        // Clear out our current accessible rolls.
        rolls.forEach(([_, idx]) => grid[idx] = '.');
        rolls = getAccessibleRolls();
    }
    console.log(`Answer part two: ${total}`);
};

solve();
