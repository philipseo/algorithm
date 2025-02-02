const fs = require("fs");

const [N, M] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split(" ")
  .map(Number);

console.log(Math.abs(N - M));
