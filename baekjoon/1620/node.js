const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const [N, M] = input.shift().split(' ').map(Number);
const poketmons = new Map(
    input.slice(0, N).map((poketmon, index) => [poketmon, index + 1])
);
const questions = input.slice(N, N + M);
const result = [];

for (const question of questions) {
    if (isNaN(question)) {
        result.push(poketmons.get(question));
    } else {
        result.push(input[Number(question) - 1]);
    }
}

console.log(result.join('\n'));
