#![warn(clippy::pedantic, clippy::nursery)]

//  0.000004957 for 1
//  0.000005335 for 10
//  0.000012792 for 100
//  0.000167352 for 1000
//  0.010188865 for 10000
//  0.020524918 for 100000
//  0.496072957 for 1000000
//  4.714067858 for 10000000
//  53.458920924 for 100000000
pub fn merge_sort<T: PartialOrd + Copy>(array: &mut Vec<T>){
    if array.len() == 2 && array[0] > array[1]{
        array.swap(0, 1);
    }else{
        let (array2, array3) = array.split_at(array.len() / 2);
        let mut array2 : Vec<T> = array2.into();
        let mut array3 : Vec<T> = array3.into();
        merge_sort(&mut array2);
        merge_sort(&mut array3);
        array.clear();
        let mut i = 0;
        let mut j = 0;
        while i < array2.len() && j < array3.len(){
            if array2[i] < array3[j]{
                array.push(array2[i]);
                i += 1;
            }else{
                array.push(array3[j]);
                j += 1;
            }
        }
        while i < array2.len(){
            array.push(array2[i]);
            i += 1;
        }
        while j < array3.len(){
            array.push(array3[j]);
            j += 1;
        }
    }
}