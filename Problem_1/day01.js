const fs = require("fs");
let hash_map = new Map();
const input = fs.readFileSync("dataset.txt", "utf-8");

input.split("\n").some((x, i) => {
    if (hash_map.has(2020 - x) && hash_map.get(2002 - x) != i) {
        console.log(x * (2020 - x));
        return true;
    } else hash_map.set(Number(x), i);
});