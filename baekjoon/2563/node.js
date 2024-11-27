const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const n = Number(input.shift());
const paper = Array.from({ length: 100 }, () => Array(100).fill(false));

for (let i = 0; i < n; i++) {
    const [x, y] = input[i].split(' ').map(Number);

    for (let darkX = 0; darkX < 10; darkX++) {
        for (let darkY = 0; darkY < 10; darkY++) {
            paper[x + darkX][y + darkY] = true;
        }
    }
}

let size = 0;

for (let row of paper) {
    size += row.filter((part) => part).length;
}

console.log(size);
