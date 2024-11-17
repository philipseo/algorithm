const fs = require('fs');

const [_n, numbers, v] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const count = numbers.split(' ').filter((number) => number === v).length;

console.log(count);