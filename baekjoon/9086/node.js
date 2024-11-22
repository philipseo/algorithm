const fs = require('fs');

const inputs = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const _T = inputs.shift().trim();

for (input of inputs) {
    console.log(`${input[0]}${input[input.length - 1]}`);
}
