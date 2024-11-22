const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map((num) => {
    return parseInt(num.split('').reverse().join(''));
});

console.log(Math.max(...input));
