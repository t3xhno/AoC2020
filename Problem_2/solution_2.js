const readline = require('readline');
const fs = require('fs');

let numberOfValidPasswords = 0;

const checkPositions = (first, second, letter, password, line) => {
    if ((password[first - 1] === letter && password[second - 1] !== letter) || (password[first - 1] !== letter && password[second - 1] === letter)) numberOfValidPasswords++;
}

const main = async () => {
    const readInterface = readline.createInterface({
        input: fs.createReadStream('./dataset.txt', 'utf-8')
    });

    for await (const line of readInterface) {
        let dataset = line.split(' ');
        let range = dataset[0].split('-');
        let letter = dataset[1].split(':')[0];
        let password = dataset[2];
        checkPositions(range[0], range[1], letter, password, line);
    }

    console.log(numberOfValidPasswords);
};

main();