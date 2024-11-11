const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim();
const regex = /^[a-z0-9]{1,50}$/;

if (!regex.test(input)) {
    console.error('The ID can only contain lowercase letters, numbers, and up to 50 characters.');
} else {
    console.log(input + '??!');
}
