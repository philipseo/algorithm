const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
let result = [];

for (const numbers of input) {
    const [first, second] = numbers.trim().split(' ').map(Number);
    const multiply = first * second;

    if (first === 0 && second === 0) {
        break;
    } else {
        if (second % first === 0) {
            result.push('factor');
        } else if (first % second === 0) {
            result.push('multiple');
        } else {
            result.push('neither');
        }
    }
}

console.log(result.join('\n'));
