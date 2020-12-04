const readline = require('readline');
const fs = require('fs');

let numberOfValidPasswords = 0;

const countOccurences = (letters, string) => {
    let occurences = 0;
    for (let letter of string) {
        if (letter === letters) occurences++;
    }
    return occurences;
};

const main = async () => {
    const readInterface = readline.createInterface({
        input: fs.createReadStream('./dataset.txt', 'utf-8')
    });

    for await (const line of readInterface) {
        let dataset = line.split(' ');
        let range = dataset[0].split('-');
        let letter = dataset[1].split(':')[0];
        let password = dataset[2];

        if (countOccurences(letter, password) >= range[0] && countOccurences(letter, password) <= range[1]) {
            console.log(`Correct line: ${line}\nTotal valid: ${numberOfValidPasswords}`);
            numberOfValidPasswords++;
        }
    }

    console.log(numberOfValidPasswords);
};

main();