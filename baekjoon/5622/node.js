const fs = require('fs');

const inputs = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('');
const dial = {
    2: 'ABC',
    3: 'DEF',
    4: 'GHI',
    5: 'JKL',
    6: 'MNO',
    7: 'PQRS',
    8: 'TUV',
    9: 'WXYZ'
};
let result = inputs.length;

for (const input of inputs) {
    for (const [key, value] of Object.entries(dial)) {
        if (value.includes(input)) {
            result += parseInt(key);
            break;
        }
    }
}

console.log(result);
