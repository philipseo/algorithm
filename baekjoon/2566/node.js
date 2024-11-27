const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
let max = 0;
let position = '';

for (let i = 0; i < input.length; i++) {
    const row = input[i].split(' ').map(Number);

    for (let j = 0; j < row.length; j++) {
        const value = row[j];
        if (value >= max) {
            max = value;
            position = `${i + 1} ${j + 1}`;
        }
    }
}

console.log(max);
console.log(position);
