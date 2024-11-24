const fs = require('fs');

const [n, m] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(BigInt);

console.log((n / m).toString());
console.log((n % m).toString());
