const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const n = parseInt(input);

console.log(`${'long '.repeat(n / 4)}int`);