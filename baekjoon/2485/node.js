const fs = require('fs');

const [_N, ...positions] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);

const gcd = (a, b) => (b === 0 ? a : gcd(b, a % b));

const gaps = [];
for (let i = 1; i < positions.length; i++) {
    gaps.push(positions[i] - positions[i - 1]);
}

let GCDs = gaps[0];
for (const gap of gaps) {
    GCDs = gcd(GCDs, gap);
}

let totalTrees = 0;
for (const gap of gaps) {
    totalTrees += gap / GCDs - 1;
}


console.log(totalTrees);
