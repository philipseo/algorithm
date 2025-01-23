const fs = require("fs");

const [T, ...PS] = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
let result = [];

for (const ps of PS) {
  let isVPS = true;
  let stack = 0;

  for (const p of ps) {
    switch (p) {
      case "(": {
        stack++;
        break;
      }
      case ")": {
        stack--;
        break;
      }
    }

    if (stack < 0) {
      isVPS = false;
      break;
    }
  }

  if (stack !== 0) {
    isVPS = false;
  }

  result.push(isVPS ? "YES" : "NO");
}

console.log(result.join("\n"));
