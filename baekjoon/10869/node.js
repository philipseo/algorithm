const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ');
const a = parseInt(input[0]);
const b = parseInt(input[1]);

console.log(a + b);
console.log(a - b);
console.log(a * b);
console.log(Math.floor(a / b));
console.log(a % b);
