const fs = require('fs');

const n = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());

console.log(n * n);
console.log(2);
