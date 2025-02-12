const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim();

const compare = {
  "1 2 3 4 5 6 7 8": "ascending",
  "8 7 6 5 4 3 2 1": "descending",
};

console.log(compare[input] || "mixed");
