const fs = require('fs');

const [firstDice, secondDice, thirdDice] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split(' ').map(Number).sort((a, b) => a - b);

if (firstDice === secondDice && secondDice === thirdDice) {
    console.log(10000 + (firstDice * 1000));
} else if (firstDice === secondDice || secondDice === thirdDice) {
    console.log(1000 + (secondDice * 100));
} else {
    console.log(thirdDice * 100);
}