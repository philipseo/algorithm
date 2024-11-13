const readline = require('readline');
const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
});

let input = [];

rl.on('line', (line) => {
    input.push(parseInt(line));
    
    if(input.length >= 2) {
        rl.close();
    }
}).on('close', () => {
    const x = input[0];
    const y = input[1];
    
    if (x === 0 || y === 0) {
        console.error('Please enter a coordinate that is a non-zero positive or negative number.');
    } else {
        if (x > 0) {
            console.log(y > 0 ? '1' : '4');
        } else {
            console.log(y > 0 ? '2' : '3');
        }
    }

    process.exit();
})
