const fs = require("fs");

const [_T, ...testCases] = fs
  .readFileSync("./dev/stdin", "utf-8")
  .trim()
  .split("\n");
const result = [];

for (const testCase of testCases) {
  let count = 0;

  const score = testCase.split("").reduce((acc, cur) => {
    if (cur === "O") {
      count++;
    } else {
      count = 0;
    }

    return acc + count;
  }, 0);

  result.push(score);
}

console.log(result.join("\n"));
