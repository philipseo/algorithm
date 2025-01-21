const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
const _N = input.shift();
const stack = [];
const result = [];

for (const line of input) {
  const [command, value] = line.split(" ").map(Number);

  switch (command) {
    case 1:
      stack.push(value);
      break;
    case 2:
      result.push(stack.length > 0 ? stack.pop() : -1);
      break;
    case 3:
      result.push(stack.length);
      break;
    case 4:
      result.push(stack.length <= 0 ? 1 : 0);
      break;
    case 5:
      result.push(stack.length > 0 ? stack[stack.length - 1] : -1);
      break;
  }
}

console.log(result.join("\n"));
