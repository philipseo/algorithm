const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const _N = Number(input[0]);
const result = input[1].split('').reduce((acc, cur) => acc + Number(cur), 0);

console.log(result);
