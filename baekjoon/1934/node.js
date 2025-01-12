const fs = require('fs');

let [_T, ...numbers] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');

function gcd(a, b) {
    while (b !== 0) {
        [a, b] = [b, a % b];
    }

    return a;
}

const result = numbers.map((number) => {
    let [A, B] = number.split(' ').map(Number);

    return (A * B) / gcd(A, B);
});

console.log(result.join('\n'));
