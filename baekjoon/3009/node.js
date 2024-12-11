const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
let xCoordinates = [];
let yCoordinates = [];

for (const coordinate of input) {
    const [x, y] = coordinate.split(' ').map(Number);
    xCoordinates.push(x);
    yCoordinates.push(y);
}

xCoordinates = xCoordinates.sort();
yCoordinates = yCoordinates.sort();

const x = xCoordinates[0] === xCoordinates[1] ? xCoordinates[2] : xCoordinates[0];
const y = yCoordinates[0] === yCoordinates[1] ? yCoordinates[2] : yCoordinates[0];

console.log(`${x} ${y}`);
