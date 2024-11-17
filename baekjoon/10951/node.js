const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');

for (let i = 0; i < input.length; i++) {
    const [a, b] = input[i].split(' ').map(Number);
    console.log(a + b);
}
