const fs = require("fs");

const [A, B, C] = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");

console.log(Number(A) + Number(B) - Number(C));
console.log(A + B - C);
