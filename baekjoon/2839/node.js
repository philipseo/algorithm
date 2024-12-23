const fs = require('fs');

let N = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());
let count = 0;

while (N > 0) {
    if (N % 5 === 0) {
        count += Math.floor(N / 5);
        break;
    } else {
        N -= 3;
        count++;
    }
}

if (N < 0) {
    count = -1;
}

console.log(count);
