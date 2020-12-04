const readline = require('readline');
const fs = require('fs');

let dataset = [];

const main = async () => {
    const readInterface = readline.createInterface({
        input: fs.createReadStream('./dataset.txt', 'utf-8')
    });

    for await (const line of readInterface) {
        dataset.push(Number(line));
    }

    for (let i = 0; i < dataset.length - 2; i++) {
        for (let j = i + 1; j < dataset.length - 1; j++) {
            for (let k = j + 1; k < dataset.length; k++) {
                if (dataset[i] + dataset[j] + dataset[k] === 2020) {
                    return dataset[i] * dataset[j];
                }
            }
        }
    }
};

main();