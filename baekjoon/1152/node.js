const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ');

console.log(input[0] === '' ? 0 : input.length);
