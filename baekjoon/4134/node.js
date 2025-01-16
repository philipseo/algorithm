const fs = require('fs');

const [_testCase, ...numbers] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);
const result = [];

const isPrime = (n) => {
    if (n < 2) return false;

    for (let i = 2; i <= Math.sqrt(n); i++) {
        if (n % i === 0) return false;
    }

    return true;
}

for (let n of numbers) {
    while (!isPrime(n)) {
        n++;
    }

    result.push(n)
}

console.log(result.join('\n'));
