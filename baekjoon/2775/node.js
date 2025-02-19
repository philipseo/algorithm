const fs = require("fs");

const [T, ...people] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split("\n");
const result = [];
let currentIndex = 0;

for (let testCase = 0; testCase < T; testCase++) {
  const k = Number(people[currentIndex++]);
  const n = Number(people[currentIndex++]);

  let floor = Array.from({ length: n }, (_, i) => i + 1);

  for (let i = 1; i <= k; i++) {
    for (let j = 1; j < n; j++) {
      floor[j] += floor[j - 1];
    }
  }

  result.push(floor[n - 1]);
}

console.log(result.join("\n"));
