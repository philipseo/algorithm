const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
const testCases = input.slice(1).map(Number);
const maxNumber = Math.max(...testCases);
const isPrimes = Array(maxNumber + 1).fill(true);
isPrimes[0] = isPrimes[1] = false;

for (let i = 2; i <= Math.sqrt(maxNumber); i++) {
  if (isPrimes[i]) {
    for (let j = 2; j <= maxNumber / i; j++) {
      isPrimes[i * j] = false;
    }
  }
}

const result = testCases.map((n) => {
  let count = 0;
  for (let i = 2; i <= n / 2; i++) {
    if (isPrimes[i] && isPrimes[n - i]) {
      count++;
    }
  }
  return count;
});

console.log(result.join("\n"));
