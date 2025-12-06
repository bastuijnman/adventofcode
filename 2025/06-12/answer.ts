import fs from 'fs/promises';

type Operation = '*' | '+';

const calc = (curr: number, value: number, operation: Operation): number => {
    switch (operation) {
        case '*': return curr * value;
        case '+': return curr + value;
    }
};

const solve = async () => {
    const buffer = await fs.readFile('input.txt');
    const rows = buffer.toString().split("\n");
    rows.pop(); // Skip empty newline at end of file. 

    // Parse out operations & rows for calculations
    const operationsInput = rows.pop();
    if (!operationsInput) {
        throw new Error('Operations input incorrect');
    }

    const operations: Operation[] = Array.from(operationsInput.matchAll(/(\*|\+)/g)).map(i => i[1] as Operation);
    const parsedRows = rows.map(row => Array.from(row.matchAll(/(\d+)/g).map(i => parseInt(i[1], 10))));

    const calculated = parsedRows.reduce((acc: number[], row: number[]) => row.map((val, i) => calc(acc[i], val, operations[i])));
    console.log(`Answer part one: ${calculated.reduce((acc, curr) => acc + curr)}`);

    // Reduce all of the cols/rows from original input (without operations) into a single array 
    let reduced: string[] = [];
    for (let i = rows[0].length - 1; i >= 0; i--) {
        for (let j = 0; j < rows.length; j++) {
            reduced[i] = (reduced[i] ?? '') + rows[j][i];
        }
    }
    reduced = reduced.map(val => val.trim());

    // Split the array on the cols with no numbers at all.
    let numsForOperations: number[][] = [[]];
    for (let i = reduced.length - 1; i >= 0; i--) {
        if (reduced[i] === '') {
            numsForOperations.unshift([]);
        } else {
            numsForOperations[0].push(parseInt(reduced[i], 10));
        }
    }

    // Perform calculation per col
    const calculatedRtl = numsForOperations.map((col, i) => col.reduce((acc, val) => calc(acc, val, operations[i])));
    console.log(`Answer part two: ${calculatedRtl.reduce((acc, curr) => acc + curr)}`);

};

solve();
