const fs = require('fs');

const [a, b, c] = fs.readFileSync('./dev/stdin', 'utf-8')
    .trim()
    .split(' ')
    .map(Number);

console.log(a + b + c);
