const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
input.pop();

const result = [];
for (const line of input) {
  const stack = [];
  let isBalanced = true;

  for (const char of line) {
    if (char === "(" || char === "[") {
      stack.push(char);
    } else if (char === ")" || char === "]") {
      const expected = char === ")" ? "(" : "[";
      if (stack.length === 0 || stack.pop() !== expected) {
        isBalanced = false;
        break;
      }
    }
  }

  result.push(isBalanced && stack.length === 0 ? "yes" : "no");
}

console.log(result.join("\n"));
