const fs = require('fs');

const inputs = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const [n, m] = inputs.shift().split(' ').map(Number);
const basket = Array.from({ length: n }, (_, index) => index + 1);

for (const input of inputs) {
   const [i, j] = input.trim().split(' ').map((number) => {
     return parseInt(number - 1);
   });
   const tempI = basket[i];
   basket[i] = basket[j];
   basket[j] = tempI;
}

console.log(basket.join(' '));
