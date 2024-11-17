const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').split('\n');
const n = parseInt(input[0]);
const values = input[1].split(' ').map(Number);

let min = values[0];
let max = values[0];

for (let i = 1; i < n; i++) {
    const value = values[i];
    if (value < min) {
        min = value;
    }
    if (value > max) {
        max = value;
    }
}

console.log(`${min} ${max}`);
