const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number);
const blackPins = [1, 1, 2, 2, 2, 8];

for (i in input) {
    blackPins[i] = blackPins[i] - input[i];
}

console.log(blackPins.join(' '));
