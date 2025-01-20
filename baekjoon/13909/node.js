const fs = require("fs");

const N = Number(fs.readFileSync("./dev/stdin", "utf-8").trim());

console.log(Math.floor(Math.sqrt(N)));
