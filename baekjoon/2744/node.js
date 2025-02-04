const fs = require("fs");

const words = fs.readFileSync("./dev/stdin", "utf-8").trim().split("");
let result = "";

for (let word of words) {
  result +=
    word === word.toUpperCase() ? word.toLowerCase() : word.toUpperCase();
}

console.log(result);
