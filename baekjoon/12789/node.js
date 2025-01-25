const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
input.shift();
const numbers = input[0].split(" ").map(Number);
const stack = [];
let currentNumber = 1;

while (numbers.length > 0 || stack.length > 0) {
  if (numbers[0] === currentNumber) {
    numbers.shift();
    currentNumber++;
  } else if (currentNumber >= stack[stack.length - 1]) {
    stack.pop();
    currentNumber++;
  } else if (numbers.length > 0) {
    stack.push(numbers.shift());
  } else {
    break;
  }
}

console.log(stack.length === 0 ? "Nice" : "Sad");
