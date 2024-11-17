const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const [_n, x] = input[0].split(' ').map(Number);
const a = input[1].split(' ').map(Number);

const result = a.filter((number) => number < x).join(' ');

console.log(result);
