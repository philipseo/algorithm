const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const _n = Number(input.shift());

const result = new Set();

for (const log of input) {
    const [name, action] = log.split(' ');

    if (action === 'enter') {
        result.add(name);
    } else if (action === 'leave')  {
        result.delete(name);
    }
} 

console.log(Array.from(result).sort().reverse().join('\n'));
