const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const [N, M] = input.shift().split(' ').map(Number);
const unheard = new Set(input.slice(0, N));
const unseen = input.slice(N, N + M);
let result = unseen.filter((name) => unheard.has(name));
result = result.sort((a, b) => a.localeCompare(b));

console.log(result.length);
console.log(result.join('\n'));
