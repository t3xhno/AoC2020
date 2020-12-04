const fs = require('fs');

const dataset = fs.readFileSync('./dataset.txt', 'utf-8');

let my_matrix = dataset.split('\n');
let tree_count = 0;
let pos_x = 0;

for (let pos_y = 0; pos_y < my_matrix.length; pos_y++) {
    if (my_matrix[pos_y][pos_x % my_matrix[0].length] === '#') tree_count++;
    pos_x += 3;
}

console.log(tree_count)