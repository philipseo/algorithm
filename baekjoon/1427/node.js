const fs = require('fs');

const result = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('').sort((a, b) => Number(b) - Number(a));

console.log(result.join(''));
