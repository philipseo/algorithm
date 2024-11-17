const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
let max = input[0];
let position = 1;

for (let i = 1; i < input.length; i++) {
    const value = parseInt(input[i]);

    if (value > max) {
        max = value;
        position = i + 1;
    }
}

console.log(max);
console.log(position);