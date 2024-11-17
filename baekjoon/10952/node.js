const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
let isEnd = false;

while(!isEnd) {
    const [a, b] = input.shift().split(' ').map(Number);

    if (a == 0 && b == 0) {
        isEnd = true;
    } else {
        console.log(a + b);
    }
}
