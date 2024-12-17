const fs = require('fs');

const n = fs.readFileSync('./dev/stdin', 'utf-8').trim();

console.log(((BigInt(n) * BigInt(n - 1) * BigInt(n - 2)) / BigInt(6)).toString());
console.log(3);
