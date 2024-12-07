const fs = require('fs');

let [_N, numbers] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const result = numbers.split(' ').filter((number) => {
    const parsedNumber = Number(number);
    if (parsedNumber === 1) {
        return false;
    } else {
        for (let i = 2; i <= Math.sqrt(parsedNumber); i++) {
            if (parsedNumber % i === 0) {
                return false;
            }
        }

        return true;
    }
});

console.log(result.length);
