const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const year = parseInt(input);

if (year < 1000 || year > 3000) {
    console.error('The year must be between 1000 and 3000.');
} else {
    console.log(year - 543);
}