const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const result = new Set(input.map((a) => parseInt(a) % 42)).size;

console.log(result);
