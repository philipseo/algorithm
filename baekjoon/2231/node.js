const fs = require('fs');

const N = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());
let M = 0;

for (let i = 1; i <= N; i++) {
    const digitSum = i
        .toString()
        .split('')
        .reduce((acc, cur) => acc + Number(cur), 0);
    const sum = i + digitSum;

    if (sum === N) {
        M = i;
        break;
    }
}

console.log(M);
