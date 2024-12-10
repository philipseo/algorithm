const fs = require('fs');

const [A, B] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);

console.log(A * B);
