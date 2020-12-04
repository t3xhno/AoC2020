const fs = require('fs');

const dataset = fs.readFileSync('./dataset.txt', 'utf-8');

const check_if_valid = (keys) => {
    for (key of ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']) {
        if (!keys.includes(key)) return false;
    }
    return true;
};

let number_of_valid = 0;

dataset.split('\n\n').forEach(passport => {
    let keys = [];
    passport.split('\n').join(' ').split(' ').forEach(element => {
        keys.push(element.split(':')[0]);
    });
    if (check_if_valid(keys)) number_of_valid++;
});

console.log(number_of_valid);