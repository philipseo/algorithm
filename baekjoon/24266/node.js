const fs = require('fs');

const n = BigInt(fs.readFileSync('./dev/stdin', 'utf-8').trim());

console.log((n * n * n).toString());
console.log(3);
