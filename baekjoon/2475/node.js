const fs = require("fs");

const sum = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split(" ")
  .map((number) => Number(number) * Number(number))
  .reduce((acc, current) => acc + current, 0);

console.log(sum % 10);
