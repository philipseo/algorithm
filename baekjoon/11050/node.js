const fs = require("fs");

const [N, K] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split(" ")
  .map(Number);

let dp = new Array(K + 1).fill(0);
dp[0] = 1;

for (let i = 1; i <= N; i++) {
  for (let j = Math.min(i, K); j > 0; j--) {
    dp[j] += dp[j - 1];
  }
}

console.log(dp[K]);
