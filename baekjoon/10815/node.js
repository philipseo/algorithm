const fs = require('fs');

const input = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');
const _N = Number(input.shift());
const nCards = input.shift().split(' ').map(Number);
const _M = Number(input.shift());
const mCards = input.shift().split(' ').map(Number);

const cards = new Set(nCards);

const result = mCards.map((card) => cards.has(card) ? 1 : 0);

console.log(result.join(' '));
