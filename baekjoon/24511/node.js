const fs = require("fs");

const inputs = fs.readFileSync(0, "utf-8").trim().split("\n");
const N = Number(inputs[0]);
const A = inputs[1].split(" ");
const B = inputs[2].split(" ");
const M = Number(inputs[3]);
const C = inputs[4].split(" ");
const result = [];

for (let i = N - 1; i >= 0; i -= 1) {
  if (result.length === M) {
    break;
  } else if (A[i] === "0") {
    result.push(B[i]);
  }
}

for (let i = 0; i < M; i += 1) {
  if (result.length === M) {
    break;
  } else {
    result.push(C[i]);
  }
}

console.log(result.join(" "));
