const fs = require('fs');

const N = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());
let layer = 1;
let range = 1;

while(range < N) {
    range += 6 * layer;
    layer++;
}

console.log(layer);
