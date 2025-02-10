const fs = require("fs");
const [_T, ...testCases] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split("\n");
const result = [];

for (const testCase of testCases) {
  const [H, _W, N] = testCase.split(" ").map(Number);
  const floor = N % H === 0 ? H : N % H;
  const ho = Math.ceil(N / H) || 1;

  result.push(`${floor}${ho < 10 ? `0${ho}` : ho}`);
}

console.log(result.join("\n"));
