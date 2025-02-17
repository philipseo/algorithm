const fs = require("fs");

const input = fs.readFileSync("./dev/stdin", "utf-8").trim().split("\n");
input.pop();

const result = input.map((number) => {
  const reversedNumber = number.split("").reverse().join("");
  return number === reversedNumber ? "yes" : "no";
});

console.log(result.join("\n"));
