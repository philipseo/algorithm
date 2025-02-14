const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
const result = [];

for (const side of input) {
  const [a, b, c] = side
    .split(" ")
    .map(Number)
    .sort((a, b) => a - b);

  if (!a && !b && !c) {
    break;
  } else {
    result.push(a * a + b * b === c * c ? "right" : "wrong");
  }
}

console.log(result.join("\n"));
