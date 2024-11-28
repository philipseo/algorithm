const fs = require('fs');

const [N, B] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number);

console.log(N.toString(B).toUpperCase());
