const fs = require("fs");

const [A, B, C] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split("\n")
  .map(Number);
let num = String(A * B * C);

for (let i = 0; i <= 9; i++) {
  console.log(num.split(i).length - 1);
}
