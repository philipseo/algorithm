const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const reverse = input.split('').reverse().join('');

console.log(input === reverse ? 1 : 0);
