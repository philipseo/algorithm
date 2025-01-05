const fs = require('fs');

const [NM, ...words] = fs
    .readFileSync('./dev/stdin', 'utf-8')
    .trim()
    .split('\n');
const [N, _M] = NM.split(' ').map(Number);
const S = new Set(words.slice(0, N));
const checkWords = words.slice(N);

let result = checkWords.reduce((count, word) => count + (S.has(word) ? 1 : 0), 0);

console.log(result);
