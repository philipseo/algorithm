const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
const N = Number(input[0]);
const sizes = input[1].split(" ").map(Number);
const [T, P] = input[2].split(" ").map(Number);

console.log(
  sizes.reduce((acc, cur) => {
    return acc + Math.ceil(cur / T);
  }, 0)
);
console.log(Math.floor(N / P), N % P);
