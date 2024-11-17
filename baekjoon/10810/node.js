const fs = require('fs');

const inputs = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const [n, _m] = inputs.shift().split(' ').map(Number);
let basket = Array(n).fill(0);

for (input of inputs) {
    const [i, j, k] = input.trim().split(' ').map(Number);
    for (let index = i - 1; index < j; index ++) {
        basket[index] = k;
    }
}

console.log(basket.join(' '));
