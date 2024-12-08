const fs = require('fs');

const [M, N] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);
const result = [];
let sum = 0;

for (let i = M; i <= N; i++) {
    if (i >= 2) {
        let isPrime = true;

        for (let j = 2; j <= Math.sqrt(i); j++) {
            if (i % j === 0) {
                isPrime = false;
                break;
            }
        }

        if (isPrime) {
            result.push(i);
            sum += i;
        }
    }
}

console.log(result.length <= 0 ? -1 : `${sum}\n${result[0]}`);
