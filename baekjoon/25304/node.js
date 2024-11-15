const fs = require('fs')

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const x = parseInt(input[0]);
const n = parseInt(input[1]);

let totalPrice = 0;

for (let i = 0; i < n; i ++) {
    const [a, b] = input[i + 2].split(' ').map(Number);
    totalPrice += a * b;
}

console.log(x === totalPrice ? 'Yes' : 'No');
