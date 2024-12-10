const fs = require('fs');

const [x, y, w, h] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number);

console.log(Math.min(h - y, w - x, y, x));
