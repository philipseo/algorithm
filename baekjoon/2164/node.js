const fs = require("fs");

const N = Number(fs.readFileSync("./dev/stdin", "utf-8").trim());
const queue = Array.from({ length: N }, (_, i) => i + 1);
let start = 0;

while (queue.length - start > 1) {
  start++;
  queue.push(queue[start]);
  start++;
}

console.log(queue[start]);
