const fs = require("fs");

const [N, K] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split(" ")
  .map(Number);
const queue = Array.from({ length: N }, (_, i) => i + 1);
const result = [];
let index = 0;

while (queue.length) {
  index = (index + K - 1) % queue.length;
  result.push(queue.splice(index, 1));
}

console.log(`<${result.join(", ")}>`);
