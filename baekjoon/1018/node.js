const fs = require('fs');

const [NM, ...board] = fs
    .readFileSync('./dev/stdin', 'utf-8')
    .trim()
    .split('\n');
const [N, M] = NM.split(' ').map(Number);

function getCount(x, y, pattern) {
    let count = 0;

    for (let i = 0; i < 8; i++) {
        for (let j = 0; j < 8; j++) {
            if (board[x + i][y + j] !== pattern[i][j]) {
                count++;
            }
        }
    }

    return count;
}

const whiteBoard = [
    'WBWBWBWB',
    'BWBWBWBW',
    'WBWBWBWB',
    'BWBWBWBW',
    'WBWBWBWB',
    'BWBWBWBW',
    'WBWBWBWB',
    'BWBWBWBW',
];
const blackBoard = [
    'BWBWBWBW',
    'WBWBWBWB',
    'BWBWBWBW',
    'WBWBWBWB',
    'BWBWBWBW',
    'WBWBWBWB',
    'BWBWBWBW',
    'WBWBWBWB',
];

let minCount = Infinity;

for (let i = 0; i <= N - 8; i++) {
    for (let j = 0; j <= M - 8; j++) {
        const whiteCount = getCount(i, j, whiteBoard);
        const blackCount = getCount(i, j, blackBoard);
        minCount = Math.min(minCount, whiteCount, blackCount);
    }
}

console.log(minCount);
