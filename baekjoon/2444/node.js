const fs = require('fs');

const input = Number(fs.readFileSync('./dev/stdin', 'utf-8'));
const loopCount = (2 * input) - 1;

for (let i = 1; i <= loopCount; i++) {
    let blank = '';
    let star = '';

    if (i <= input) {
        blank = ' '.repeat(input - i);
        star = '*'.repeat(i + i - 1);
    } else {
        const reverseLoopCount = loopCount - i + 1;
        blank = ' '.repeat(i - input);
        star = '*'.repeat(reverseLoopCount + reverseLoopCount - 1);
    }

    console.log(`${blank}${star}`);
}
