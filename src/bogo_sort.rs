// 0.000005871 for 1
// 0.000034767 for 2
// 0.000016156 for 4
// 0.007449113 for 8
pub fn bogo_sort<T: PartialOrd>(values: &mut Vec<T>){
    while !values.is_sorted() {
        let mut new_values = Vec::with_capacity(values.len());
        for _ in 0..values.len(){
            let index = rand::random::<usize>() % values.len();
            new_values.push(values.swap_remove(index));
        }
        *values = new_values;
    }
}
