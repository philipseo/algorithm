const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().toUpperCase().split('');

const words = input.reduce((acc, cur) => {
    acc[cur] ? acc[cur] += 1 : acc[cur] = 1
    return acc;
}, {});
const sortedWords = Object.entries(words).sort(([_aWord, a], [_bWord, b]) => b - a);

console.log(sortedWords.length !== 1 && sortedWords[0][1] === sortedWords[1][1] ? '?' : sortedWords[0][0]);
