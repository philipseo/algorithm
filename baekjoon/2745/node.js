const fs = require('fs');

const [N, B] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ');

console.log(parseInt(N, B));
