import fs from 'fs/promises';

type Beam = {
    index: number,
    paths: number,
};

const splitBeam = (beam: Beam, splitters: number[]): Beam[] => {
    if (!splitters.includes(beam.index)) {
        return [beam];
    }
    return [{ index: beam.index - 1, paths: beam.paths }, { index: beam.index + 1, paths: beam.paths }];
}


const solve = async () => {
    const buffer = await fs.readFile('input.txt');
    const lines = buffer.toString().split("\n");

    // Get rid of newline after file end.
    lines.pop();

    // Construct initial start of beam.
    let beams: Beam[] = [];
    let start = lines.shift();
    if (start) {
        beams.push({ index: start.indexOf('S'), paths: 1 });
    }

    let splits = 0;
    for (let i = 0; i < lines.length; i++) {
        const splitters = Array.from(lines[i].matchAll(/(\^)/g)).map(splitter => splitter.index);

        // Count the actual splits that happen
        splits += beams.reduce((acc, beam) => acc + (splitters.includes(beam.index) ? 1 : 0), 0);

        // Make the splits
        beams = beams.flatMap(beam => splitBeam(beam, splitters));

        // Calculate possible paths
        beams = beams.map(beam => {
            const sum = beams.filter(b => b.index === beam.index).reduce((acc, b) => acc + b.paths, 0);
            return { index: beam.index, paths: sum };
        });

        // Merge beams
        beams = beams.filter((beam, index) => beams.findIndex(value => value.index === beam.index) === index);
    }
    console.log(`Answer part one: ${splits}`);
    console.log(`Answer part two: ${beams.reduce((acc, beam) => acc + beam.paths, 0)}`);


};

solve();
