const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const maxLength = Math.max(...input.map((value) => value.length));
let result = '';

for (let i = 0; i < maxLength; i++) {
    for (let j = 0; j < 5; j++) {
        if (input[j][i]) {
            result += input[j][i];
        }
    }
}

console.log(result);
