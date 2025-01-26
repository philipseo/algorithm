const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
const _N = input.shift();
const queue = [];
const result = [];
let index = 0;

for (const line of input) {
  const [command, value] = line.split(" ");

  switch (command) {
    case "push":
      queue.push(value);
      break;
    case "pop":
      result.push(index < queue.length ? queue[index++] : -1);
      break;
    case "size":
      result.push(queue.length - index);
      break;
    case "empty":
      result.push(index < queue.length ? 0 : 1);
      break;
    case "front":
      result.push(index < queue.length ? queue[index] : -1);
      break;
    case "back":
      result.push(index < queue.length ? queue[queue.length - 1] : -1);
      break;
  }
}

console.log(result.join("\n"));
