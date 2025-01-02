const fs = require('fs');

const [_N, ...members] = fs.readFileSync('./dev/stdin', 'utf-8').trim().split('\n');

const result = members
    .map((member) => {
        const [age, name] = member.split(' ');

        return {
            age: Number(age),
            name,
        };
    })
    .sort((a, b) => a.age - b.age)
    .map((member) => `${member.age} ${member.name}`);

console.log(result.join('\n'));
