const fs = require("fs");

const [a, b] = fs
  .readFileSync("/dev/stdin", "utf-8")
  .trim()
  .split(" ")
  .map(Number);

function gcd(a, b) {
  while (b !== 0) {
    [a, b] = [b, a % b];
  }

  return a;
}

const greatestCommonDivisor = gcd(a, b);

console.log(greatestCommonDivisor);
console.log((a * b) / greatestCommonDivisor);
