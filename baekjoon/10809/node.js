const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('');
const alphabet = 'abcdefghijklmnopqrstuvwxyz';

const result = alphabet.split('').map((char) => input.indexOf(char)).join(' ');

console.log(result);
