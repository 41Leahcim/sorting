import * as algorithms from "./algorithms.js";
const length = 100000;

let values = [];

for(let i = 0;i < length;i++){
    values.push(length - i);
}

const start = new Date();
algorithms.selection_sort(values);
const performance = (new Date() - start) / 1000.0;

/*values.forEach(value => {
    console.log(value);
});*/

console.log(performance);
