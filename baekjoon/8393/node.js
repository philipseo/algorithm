const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const n = parseInt(input);

let result = 0;

for (let i = 1; i <= n; i ++) {
    result += i;
}

console.log(result);