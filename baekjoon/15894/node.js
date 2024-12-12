const fs = require('fs');

const input = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());

console.log(input * 4);
