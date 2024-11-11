const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ');
const a = parseInt(input[0]);
const b = parseInt(input[1]);
const c = parseInt(input[2]);

if (a < 2 || a > 10000 || b < 2 || b > 10000 || c < 2 || c > 10000) {
    console.error('a, b and c must be between 2 and 10000');
} else {
    console.log((a + b) % c);
    console.log(((a % c) + (b % c)) % c);
    console.log((a * b) % c);
    console.log(((a % c) * (b % c)) % c);
}
