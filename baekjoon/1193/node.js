const fs = require('fs');

let X = Number(fs.readFileSync('./dev/stdin', 'utf-8').trim());
let index = 1;

while (X > index) {
    X -= index++;
}

if (index % 2 === 0) {
    console.log(`${X}/${index - X + 1}`);
} else {
    console.log(`${index - X + 1}/${X}`)
}
