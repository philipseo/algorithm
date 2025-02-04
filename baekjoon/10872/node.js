const fs = require("fs");

const N = Number(fs.readFileSync("./dev/stdin", "utf-8").trim());
let result = 1;

for (let i = 1; i <= N; i++) {
  result *= i;
}

console.log(result);
