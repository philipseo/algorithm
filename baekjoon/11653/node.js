const fs = require('fs');

let N = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());
let divisor = 2;
const result = [];

while (N > 1) {
    if (N % divisor === 0) {
        result.push(divisor);
        N = N / divisor;
        divisor = 2;
    } else {
        divisor++;
    }
}

console.log(result.join('\n'));
