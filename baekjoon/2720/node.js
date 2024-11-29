const fs = require('fs');

const [T, ...changes] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n').map(Number);

for (let C of changes) {
    const quarter = Math.floor(C / 25);
    C %= 25;
    
    const dime = Math.floor(C / 10);
    C %= 10;

    const nickel = Math.floor(C / 5);
    const penny = C % 5;

    console.log(`${quarter} ${dime} ${nickel} ${penny}`);
}
