const algorithms = require ("./algorithms");
const length = 10000000;

let values = [];

for(let i = 0;i < length;i++){
    values.push(length - i);
}

const start = new Date();
algorithms.merge_sort(values);
const performance = (new Date() - start) / 1000.0;

values.forEach(value => {
    console.log(value);
});

console.log(performance);
