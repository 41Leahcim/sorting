// 0.000004690 for 1
// 0.000026638 for 10
// 0.000085314 for 100
// 0.001675547 for 1000
// 0.080591259 for 10000
// 7.505769325 for 100000
pub fn insertion_sort<T: PartialOrd>(values: &mut Vec<T>){
    let mut i = 1;
    while i < values.len(){
        if values[i] < values[i - 1]{
            values.swap(i, i - 1);
            if i > 1{
                i -= 1;
            }else{
                i += 1;
            }
        }else{
            i += 1;
        }
    }
}