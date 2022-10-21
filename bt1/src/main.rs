fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [1, 2, 4, 5];

    for (i, item) in org_arr.iter().enumerate() {
        let x:i32 = *item;
        let mut k = i;
        if x == sub_arr[0] {
            let mut j = 0;
            while j < sub_arr.len() && org_arr[k] == sub_arr[j] {
                k += 1;
                j += 1;
            }
            if j == sub_arr.len() {
                println!("True")
            }
        }
    }
}
