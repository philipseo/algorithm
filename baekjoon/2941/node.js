const fs = require('fs');

let input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const croatiaAlphabets = ['c=', 'c-', 'dz=', 'd-', 'lj', 'nj', 's=', 'z='];

for (const croatiaAlphabet of croatiaAlphabets) {
    input = input.replaceAll(croatiaAlphabet, "C");
}

console.log(input.length)
