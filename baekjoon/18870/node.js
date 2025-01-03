const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const _N = input.shift();
const coordinates = input[0].split(' ').map(Number);
const coordinatesMap = new Map();

Array.from(new Set(coordinates))
    .sort((a, b) => a - b)
    .forEach((value, index) => coordinatesMap.set(value, index));

const result = coordinates.map((value) => coordinatesMap.get(value));

console.log(result.join(' '));
