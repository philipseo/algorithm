const fs = require('fs');

let [A, B, V] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number);

console.log(Math.ceil((V - B) / (A - B)));
