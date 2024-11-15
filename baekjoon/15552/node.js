const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const t = parseInt(input[0]);
let result = '';

for(let i = 1; i <= t; i++) {
    const [a, b] = input[i].split(' ').map(Number);
    result += `${a + b}\n`;
}

console.log(result);
