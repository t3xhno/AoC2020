const readline = require('readline');
const fs = require('fs');

let dataset = [];

const main = (async () => {
    const readInterface = readline.createInterface({
        input: fs.createReadStream('./dataset.txt', 'utf-8')
    });

    for await (const line of readInterface) {
        dataset.push(Number(line));
    }

    for (let i = 0; i < dataset.length - 1; i++) {
        for (let j = i + 1; j < dataset.length; j++) {
            if (dataset[i] + dataset[j] === 2020) {
                console.log(`${dataset[i]} + ${dataset[j]} (sum = ${dataset[i] + dataset[j]}) = ${dataset[i] * dataset[j]}`);
                return dataset[i] * dataset[j];
            }
        }
    }
})();