const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map((value) => value.split(' ').map(Number));
const [n, m] = input.shift();
const A = input.slice(0, n);
const B = input.slice(n);
let result = '';

for (let i = 0; i < n; i++) {
    for (let j = 0; j < m; j++) {
        result += `${A[i][j] + B[i][j]}${j !== (m - 1) ? ' ' : ''}`;
    }
    result += i !== n - 1 ? '\n' : '';
}

console.log(result);
