const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const [aLength, bLength] = input.shift().split(' ').map(Number);
const A = new Set(input.shift().split(' ').map(Number));
const B = new Set(input.shift().split(' ').map(Number));

let count = 0;

for (const a of A) {
    if (!B.has(a)) {
        count++;
    }
}

for (const b of B) {
    if (!A.has(b)) {
        count++;
    }
}

console.log(count);
