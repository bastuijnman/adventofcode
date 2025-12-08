import fs from 'fs/promises';

type JunctionBox = {
    x: number,
    y: number,
    z: number,
};

type Circuit = {
    boxIndices: number[]
};

const parseJunctionBox = (line: string): JunctionBox => {
    const values = line.match(/(\d+)/g);
    if (!values || values.length < 3) {
        throw new Error('Incorrect line supplied for junction box');
    }
    return { x: parseInt(values[0], 10), y: parseInt(values[1], 10), z: parseInt(values[2], 10) };
};

// Calculate euclidian distance between boxes.
const distance = (a: JunctionBox, b: JunctionBox): number => {
    return Math.hypot(b.x - a.x, b.y - a.y, b.z - a.z);
};

// Filter out only unique pairs.
const unique = (values: [number, number][]): [number, number][] => {
    const cache = new Set();
    return values.filter(pair => {
        pair.sort();
        const key = `${pair[0]},${pair[1]}`;
        if (cache.has(key)) {
            return false;
        }
        cache.add(key);
        return true;
    });
}

const solve = async () => {
    const buffer = await fs.readFile('input.txt');
    const lines = buffer.toString().split("\n");
    lines.pop(); // Get rid of newline at EOF

    const boxes: JunctionBox[] = lines.map(parseJunctionBox);
    let circuits: Circuit[] = boxes.map((_, index) => ({ boxIndices: [index] }));

    // Generate all possible distance pairs
    let distances: number[][] = boxes
        .flatMap((_, a) => boxes.map((_, b) => [a, b]))
        .filter(indices => indices[0] !== indices[1]);

    // Filter on unique pairs and sort by their actual distance.
    distances = unique(distances as [number, number][])
        .sort((a, b) => distance(boxes[a[0]], boxes[a[1]]) - distance(boxes[b[0]], boxes[b[1]]));

    for (let i = 0; i < distances.length; i++) {

        // Find circuits for both entries in the distance pair.
        const firstIndex = circuits.findIndex(circuit => circuit.boxIndices.includes(distances[i][0]));
        const secondIndex = circuits.findIndex(circuit => circuit.boxIndices.includes(distances[i][1]));

        // Merge two circuits when they exist and get rid of one of them.
        if (circuits[firstIndex].boxIndices.length >= 1 && circuits[secondIndex].boxIndices.length >= 1 && firstIndex !== secondIndex) {
            circuits[firstIndex].boxIndices = circuits[firstIndex].boxIndices.concat(circuits[secondIndex].boxIndices);
            circuits.splice(secondIndex, 1);
        }


        // We've reached the part one answer when we processed 1000 distances
        if (i === 1000) {

            // Sort and take the biggest 3 circuits
            const total = circuits.sort((a, b) => b.boxIndices.length - a.boxIndices.length).slice(0, 3).map(circuit => circuit.boxIndices.length).reduce((prev, curr) => prev * curr);
            console.log(`Answer part one: ${total}`);
        }

        // It's all converged to one circuit, part two done.
        if (circuits.length === 1) {
            const total = boxes[distances[i][0]].x * boxes[distances[i][1]].x;
            console.log(`Answer part two: ${total}`);
            return;
        }
    }

};

solve();
