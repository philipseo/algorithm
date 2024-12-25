const fs = require('fs');

const input = fs.readFileSync('/dev/stdin', 'utf-8').trim().split('\n').map(Number);
const average = input.reduce((acc, cur) => acc + cur, 0) / input.length;
const sortedInput = input.sort((a, b) => a - b);
const median = sortedInput[2];

console.log(`${average}\n${median}`);
