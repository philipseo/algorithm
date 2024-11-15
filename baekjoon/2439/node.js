const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const n = parseInt(input);
let result = '';

for (let i = 1; i <= n; i++) {
    result += `${' '.repeat(n-i)}${'*'.repeat(i)}${i !== n ? '\n' : ''}`
}

console.log(result);