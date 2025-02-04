const fs = require("fs");

const N = Number(fs.readFileSync("./dev/stdin", "utf-8").trim());
const result = [];

for (i = 1; i <= N; i++) {
  result.push(i);
}

console.log(result.join("\n"));
