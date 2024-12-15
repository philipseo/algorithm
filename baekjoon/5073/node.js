const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');

const result = [];

for (const line of input) {
    const sides = line.split(' ').map(Number);
    
    if (sides[0] === 0 && sides[1] === 0 && sides[2] === 0) {
        break;
    } else {
        const [a, b, c] = sides.sort((a, b) => a - b);
        
        if (a + b <= c) {
            result.push('Invalid');
        } else if (a === b && b === c) {
            result.push('Equilateral');
        } else if (a === b || b === c || a === c) {
            result.push('Isosceles');
        } else {
            result.push('Scalene');
        }
    }
}

console.log(result.join('\n'));
