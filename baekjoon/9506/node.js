const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);

for (const n of input) {
    if (n === -1) {
        break;
    } else {
        let sum = 0;
        let array = [];

        for (let i = 1; i < n; i++) {
            if (n % i === 0) {
                sum += i;
                array.push(i);
            }
        }

        if (n === sum) {
            console.log(`${n} = ${array.join(' + ')}`);
        } else {
            console.log(`${n} is NOT perfect.`);
        }
    }
}
