const fs = require('fs');

const [_N, ...coordinates] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');

const xCoordinates = [];
const yCoordinates = [];

for (const coordinate of coordinates) {
    const [x, y] = coordinate.split(' ').map(Number);
    xCoordinates.push(x);
    yCoordinates.push(y);
}

const width = Math.max(...xCoordinates) - Math.min(...xCoordinates);
const height = Math.max(...yCoordinates) - Math.min(...yCoordinates);

console.log(width * height);
