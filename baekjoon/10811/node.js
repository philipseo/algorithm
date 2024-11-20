const fs = require('fs');

const inputs = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const [n, m] = inputs.shift().split(' ').map(Number);
const baskets = Array.from({ length: n }, (_, i) => i + 1);

for (input of inputs) {
    const [i, j] = input.split(' ').map(Number);
    baskets.slice(i - 1, j).reverse().forEach((value, idx) => {
      baskets[i - 1 + idx] = value;
    });
}

console.log(baskets.join(' '));
