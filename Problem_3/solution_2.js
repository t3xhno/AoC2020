const fs = require('fs');

const dataset = fs.readFileSync('./dataset.txt', 'utf-8');

let my_matrix = dataset.split('\n');
const steps = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];
let total_trees = 1;

for (step of steps) {
    let pos_x = 0;
    let tree_count = 0;
    let step_x = step[0];
    let step_y = step[1];
    for (let pos_y = 0; pos_y < my_matrix.length; pos_y += step_y) {
        if (my_matrix[pos_y][pos_x % my_matrix[0].length] === '#') tree_count++;
        pos_x += step_x;
    }
    total_trees *= tree_count;
}

console.log(total_trees);