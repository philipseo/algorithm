const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const t = parseInt(input[0]);

for (let i = 1; i <= t; i++) {
    let [a, b] = input[i].split(' ');
    console.log(parseInt(a) + parseInt(b));
}