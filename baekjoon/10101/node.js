const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);

const sum = input.reduce((acc, cur) => acc + cur, 0);

if (sum !== 180) {
    console.log('Error');
} else if (input[0] === 60 && input[1] === 60 && input[2] === 60) {
    console.log('Equilateral');
} else if (input[0] === input[1] || input[0] === input[2] || input[1] === input[2]) {
    console.log('Isosceles');
} else {
    console.log('Scalene');
}
