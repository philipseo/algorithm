const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);
const totalStudents = 30;
const notSubmitted = [];

for (let i = 1; i  <= totalStudents; i++) {
    if (!input.includes(i)) {
        notSubmitted.push(i);
    }
}

notSubmitted.sort((a, b) => a - b);
console.log(`${notSubmitted[0]}\n${notSubmitted[1]}`);
