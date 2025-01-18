const fs = require("fs");

const testCases = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split("\n")
  .map(Number);
testCases.pop();

const MAX = 123456 * 2;
const isPrimes = Array(MAX + 1).fill(true);
isPrimes[0] = isPrimes[1] = false;

for (let i = 2; i <= Math.sqrt(MAX); i++) {
  if (isPrimes[i]) {
    for (let j = i * i; j <= MAX; j += i) {
      isPrimes[j] = false;
    }
  }
}

const result = testCases.map((n) => {
  let count = 0;
  for (let i = n + 1; i <= 2 * n; i++) {
    if (isPrimes[i]) count++;
  }
  return count;
});

console.log(result.join("\n"));
