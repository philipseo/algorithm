const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const n = parseInt(input[0]);
const scores = input[1].split(' ').map(Number);
const m = Math.max(...scores);
const totalScore = scores.reduce((acc, score) => {
    return acc + (score / m * 100);
}, 0);

console.log(totalScore / n);