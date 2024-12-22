const fs = require('fs');

const N = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());

let count = 0;
let num = 665;

while (count !== N) {
    num++;
    if (String(num).includes("666")) {
        count++;
    }
}

console.log(num);
