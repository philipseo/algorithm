const fs = require('fs');

let [a, b, c] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number).sort((a, b) => a - b);

while ((a + b) <= c) {
    c--;
}

console.log(a + b + c);
