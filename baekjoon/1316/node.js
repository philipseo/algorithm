const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
let n = parseInt(input.shift());

for (const word of input) {
    const seen = new Set();
    let previousChar = null;

    for (const char of word) {
        if (char !== previousChar) {
            if (seen.has(char)) {
                n--;
                break;
            } else {
                seen.add(char);
            }
        }
        previousChar = char;
    }
}

console.log(n);
