const fs = require('fs');

const [N, ...numbers] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);

numbers.sort((a, b) => a - b);

console.log(numbers.join('\n'));
