const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const [numerator1, denominator1] = input[0].split(' ').map(Number);
const [numerator2, denominator2] = input[1].split(' ').map(Number);

const gcd = (a, b) => (b === 0 ? a : gcd(b, a % b));

const numerator = numerator1 * denominator2 + numerator2 * denominator1;
const denominator = denominator1 * denominator2;
const divisor = gcd(numerator, denominator);

console.log(numerator / divisor, denominator / divisor);
