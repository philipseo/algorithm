const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const _N = Number(input.shift());

const coordinates = input.map(line => line.split(' ').map(Number));

coordinates.sort((a, b) => a[0] - b[0] || a[1] - b[1]);

console.log(coordinates.map(coord => coord.join(' ')).join('\n'));
