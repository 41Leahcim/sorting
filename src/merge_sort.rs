//  0.000004957 for 1
//  0.000005335 for 10
//  0.000012792 for 100
//  0.000167352 for 1000
//  0.010188865 for 10000
//  0.020524918 for 100000
//  0.496072957 for 1000000
//  4.714067858 for 10000000
//  53.458920924 for 100000000
pub fn merge_sort<T: PartialOrd + Copy>(a: &mut Vec<T>){
    if a.len() <= 2{
        if a.len() == 2 && a[0] > a[1]{
            a.swap(0, 1);
        }
    }else{
        let (b, c) = a.split_at(a.len() / 2);
        let mut b : Vec<T> = b.into();
        let mut c : Vec<T> = c.into();
        merge_sort(&mut b);
        merge_sort(&mut c);
        a.clear();
        let mut i = 0;
        let mut j = 0;
        while i < b.len() && j < c.len(){
            if b[i] < c[j]{
                a.push(b[i]);
                i += 1;
            }else{
                a.push(c[j]);
                j += 1;
            }
        }
        while i < b.len(){
            a.push(b[i]);
            i += 1;
        }
        while j < c.len(){
            a.push(c[j]);
            j += 1;
        }
    }
}