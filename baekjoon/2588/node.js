const fs = require('fs');

const input = fs.readFileSync('/dev/stdin', 'utf-8').trim().split('\n');
const a = parseInt(input[0]);
const b = parseInt(input[1]);

console.log(a * (b % 10));
console.log(a * Math.floor((b % 100) / 10));
console.log(a * Math.floor(b / 100));
console.log(a * b);
