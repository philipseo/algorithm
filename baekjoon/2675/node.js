const fs = require('fs');

const inputs = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const _T = parseInt(inputs.shift());

for (input of inputs) {
    const [R, S] = input.trim().split(' ');
    const P = S.split('').reduce((acc, cur) => acc + cur.repeat(parseInt(R, 10)), '');
    
    console.log(P);
}
