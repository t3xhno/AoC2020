const fs = require('fs');

const dataset = fs.readFileSync('./dataset.txt', 'utf-8');

const valid_byr = (byr) => {
    return (byr <= 2002 && byr >= 1920) ? true : false;
};

const valid_iyr = (iyr) => {
    return (iyr <= 2020 && iyr >= 2010) ? true : false;
};

const valid_eyr = (eyr) => {
    return (eyr <= 2030 && eyr >= 2020) ? true : false;
};

const valid_hgt = (hgt) => {
    if (hgt.slice(-2) === 'cm') {
        return (hgt.slice(0, 3) <= 193 && hgt.slice(0, 3) >= 150) ? true : false;
    } else if (hgt.slice(-2) === 'in') {
        return (hgt.slice(0, 2) <= 76 && hgt.slice(0, 2) >= 59) ? true : false;
    }
    return false;
};

const valid_hcl = (hcl) => {
    return (/^#[0-9a-f]{6}/.exec(hcl)) ? true : false;
};

const valid_ecl = (ecl) => {
    return (['amb', 'blu', 'brn', 'gry', 'grn', 'hzl', 'oth'].includes(ecl)) ? true : false;
};

const valid_pid = (pid) => {
    return (pid.length === 9 && Number(pid)) ? true : false;
};

const check_if_valid = (keys) => {
    for (key of ['byr', 'iyr', 'eyr', 'hgt', 'hcl', 'ecl', 'pid']) {
        if (!keys.includes(key)) return false;
    }
    return true;
};

let number_of_valid = 0;

dataset.split('\n\n').forEach(passport => {
    let passport_data = {};
    let passport_keys = [];
    passport.split('\n').join(' ').split(' ').forEach(element => {
        passport_data[element.split(':')[0]] = element.split(':')[1];
        passport_keys.push(element.split(':')[0]);
    });
    if (check_if_valid(passport_keys)) {
        (valid_byr(passport_data['byr']) && valid_ecl(passport_data['ecl'])
            && valid_eyr(passport_data['eyr']) && valid_hcl(passport_data['hcl'])
            && valid_hgt(passport_data['hgt']) && valid_iyr(passport_data['iyr'])
            && valid_pid(passport_data['pid'])) ? number_of_valid++ : 0;
    }
});

console.log(number_of_valid);