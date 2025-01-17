const fs = require("fs");

let [N, M] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split(" ")
  .map(Number);

const primes = new Array(M + 1).fill(false);

for (let i = 2; i <= Math.sqrt(M); i++) {
  if (!primes[i]) {
    for (let j = i * i; j <= M; j += i) {
      primes[j] = true;
    }
  }
}

const result = [];

for (let i = N; i <= M; i++) {
  if (i > 1 && !primes[i]) {
    result.push(i);
  }
}

console.log(result.join("\n"));
