const fs = require('fs');

const [a, b, c, d, e, f] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number);
let result = null;

for (let x = -999; x < 1000; x++) {
    for (let y = -999; y < 1000; y++) {
        if (a * x + b * y === c && d * x + e * y === f) {
            result = `${x} ${y}`;
            break;
        }
    }

    if (result) {
        break;
    }
}

console.log(result);
