const fs = require('fs');

const [N, K] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number);
let count = 0;
let result = 0;

for (let i = 1; i <= N; i++) {
    if (N % i === 0) {
        count++;
        if (count === K) {
            result = i;
            break;
        }
    }
}

console.log(result);
