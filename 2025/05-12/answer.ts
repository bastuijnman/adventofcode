import fs from 'fs/promises';

type Range = [number, number];

const parseRange = (range: string): Range => {
    const [start, end] = range.split('-').map(r => parseInt(r, 10));
    return [start, end];
}

// Keep collapsing ranges until we cannot collapse anymore
const collapseFreshRanges = (freshIds: Array<Range | null>) => {
    // Collapse fresh IDs into unique ranges
    for (let i = 0; i < freshIds.length; i++) {
        for (let j = 0; j < freshIds.length; j++) {

            // Ignore the same iteration and any already processed ranges
            if (i === j || freshIds[i] === null || freshIds[j] === null) {
                continue;
            }

            const [aStart, aEnd] = freshIds[i] as Range;
            const [bStart, bEnd] = freshIds[j] as Range;

            // If ranges overlap condense into one.
            if ((aStart >= bStart && aStart <= bEnd) || (aEnd >= bStart && aEnd <= bEnd)) {

                // Condense into the forward range
                freshIds[j] = [
                    Math.min(aStart, bStart),
                    Math.max(aEnd, bEnd),
                ];

                // Get rid of curent one
                freshIds[i] = null;
            }
        }
    }
    return freshIds.filter(range => range !== null);
}

const solve = async () => {
    const buffer = await fs.readFile('input.txt');
    let [rangeInput, inventoryInput] = buffer.toString().split("\n\n");

    // Split input into fresh ranges and inventory IDs
    const freshRanges: Array<Range | null> = rangeInput.split("\n").map(parseRange);
    const inventory = inventoryInput.split("\n").map(i => parseInt(i, 10));

    // Find inventory within fresh ranges
    const fresh = inventory.filter(i => freshRanges.filter(range => range !== null).some(range => i >= range[0] && i <= range[1]));
    console.log(`Answer part one: ${fresh.length}`);

    // Find all possible fresh IDs
    const totalFreshIds = collapseFreshRanges(freshRanges).reduce((acc, range) => acc + (range[1] - range[0]) + 1, 0);
    console.log(`Answer part two: ${totalFreshIds}`);
};

solve();
