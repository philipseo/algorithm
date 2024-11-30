const fs = require('fs');

const input = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());

console.log(Math.pow(Math.pow(2, input) + 1, 2));
