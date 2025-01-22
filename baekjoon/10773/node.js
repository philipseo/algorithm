const fs = require("fs");

const [_N, ...numbers] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split("\n")
  .map(Number);
const stack = [];

for (const number of numbers) {
  if (number === 0) {
    stack.pop();
  } else {
    stack.push(number);
  }
}

const result = stack.reduce((acc, cur) => acc + cur, 0);

console.log(result);
