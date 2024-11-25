const fs = require('fs');

const inputs = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');

const gradeToPoint = {
    'A+': 4.5,
    'A0': 4.0,
    'B+': 3.5,
    'B0': 3.0,
    'C+': 2.5,
    'C0': 2.0,
    'D+': 1.5,
    'D0': 1.0,
    'F': 0.0,
}

let totalPoint = 0;
let totalAvgMajorPoint = 0;

for (const input of inputs) {
    const [_, point, grade] = input.split(' ');
    const floatPoint = parseFloat(point);
    
    if (grade === 'P') {
        continue;
    } else {
        totalPoint += floatPoint;
        totalAvgMajorPoint += floatPoint * gradeToPoint[grade];
    }
}

console.log((totalAvgMajorPoint / totalPoint).toFixed(6));
