const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
let words = new Set();

for (let i = 0; i < input.length; i++) {
    for (let j = i + 1; j <= input.length; j++) {
        words.add(input.slice(i, j));
    }
}

console.log(words.size);
