const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const currentTime = input[0].split(' ');
let currentHour = parseInt(currentTime[0]);
let currentMinute = parseInt(currentTime[1]);
const cookTime = parseInt(input[1]);

currentMinute += cookTime;

if (currentMinute >= 60) {
    currentHour += Math.floor(currentMinute / 60);
    currentMinute %= 60;

    if (currentHour > 23) {
        currentHour -= 24
    }
}

console.log(`${currentHour} ${currentMinute}`);