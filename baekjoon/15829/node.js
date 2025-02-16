const fs = require("fs");

const [L, words] = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
const r = 31;
const M = 1234567891;
let hash = 0;
let power = 1;

for (let i = 0; i < L; i++) {
  hash += (words[i].charCodeAt(0) - 96) * power;
  hash %= M;
  power *= r;
  power %= M;
}

console.log(hash);
