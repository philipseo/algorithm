const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ');
let hour = parseInt(input[0]);
let minute = parseInt(input[1]);

minute -= 45;

if (minute < 0) {
    minute += 60;
    hour--;
    
    if (hour < 0) {
        hour = 23;
    }
}

console.log(`${hour} ${minute}`);