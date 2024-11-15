const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const n = parseInt(input);

for (i = 1; i < 10; i++) {
    console.log(`${n} * ${i} = ${n * i}`);
}