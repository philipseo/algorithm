const fs = require('fs');

const [_N, ...words] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');

const sortedWords = Array.from(new Set(words)).sort((a, b) => {
    if (a.length > b.length) {
        return 1;
    } else if (a.length < b.length) {
        return -1;
    } else {
        return a.localeCompare(b);
    }
});

console.log(sortedWords.join("\n"));
