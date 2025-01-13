const fs = require('fs');

const [A, B] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number);

function gcd(a, b) {
    while (b !== 0) {
        [a, b] = [b, a % b];
    }

    return a;
}

console.log((A * B) / gcd(A, B));
