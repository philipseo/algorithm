const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const year = parseInt(input);

if (year < 1 || year > 4000) {
    console.error('Please enter a year between 1 and 4000.');
} else {
    if ((year % 4 === 0 && year % 100 !== 0) || year % 400 === 0) {
        console.log("1");
    } else {
        console.log("0");
    }
}
