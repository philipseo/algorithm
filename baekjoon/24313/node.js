const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');

const [a1, a0] = input[0].split(' ').map(Number);
const c = Number(input[1]);
const n0 = Number(input[2]);

const fn = (a1 * n0) + a0;
const gn = c * n0;

if (a1 <= c && fn <= gn) {
    console.log(1);
} else {
    console.log(0);
}
