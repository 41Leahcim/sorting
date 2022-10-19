//    100 : 0.007
//   1000 : 0.008
//  10000 : 0.171
// 100000 : 16.639
exports.bubble_sort = (values) => {
    for(let i = 0;i < values.length;i++){
        for(let j = 1;j < values.length - i;j++){
            if(values[j - 1] > values[j]){
                [values[j - 1], values[j]] = [values[j], values[j - 1]];
            }
        }
    }
}

//    100 : 0.005
//   1000 : 0.006
//  10000 : 0.253
// 100000 : 24.051
exports.insertion_sort = (values) => {
    let i = 1;
    while(i < values.length){
        if(values[i] < values[i - 1]){
            [values[i - 1], values[i]] = [values[i], values[i - 1]]
            if(i > 1){
                i--;
            }else{
                i++;
            }
        }else{
            i++;
        }
    }
}

//     1000 : 0.003
//    10000 : 0.015
//   100000 : 0.045
//  1000000 : 0.257
// 10000000 : 2.67
exports.merge_sort = (array) => {
    if(array.length == 2 && array[0] > array[1]){
        [array[0], array[1]] = [array[1], array[0]]
    }else if(array.length > 2){
        let [array2, array3] = [array.slice(0, array.length / 2), array.slice(array.length / 2)];
        this.merge_sort(array2)
        this.merge_sort(array3)
        let [i, j, k] = [0, 0, 0];
        while(i < array2.length && j < array3.length){
            if(array2[i] < array3[j]){
                array[k++] = array2[i++];
            }else{
                array[k++] = array3[j++];
            }
        }
        while(i < array2.length){
            array[k++] = array2[i++];
        }
        while(j < array3.length){
            array[k++] = array3[j++];
        }
    }
}

//   1000 : 0.007
//  10000 : 0.039
// 100000 : 3.64
exports.selection_sort = (values) => {
    for(let i = 0;i < values.length;i++){
        let lowest_index = i;
        for(let j = i + 1;j < values.length;j++){
            if(values[j] < values[lowest_index]){
                lowest_index = j;
            }
        }
        [values[i], values[lowest_index]] = [values[lowest_index], values[i]];
    }
}
