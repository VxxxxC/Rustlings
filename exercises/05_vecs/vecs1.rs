fn array_and_vec() -> ([i32; 4], Vec<i32>) {
    let a = [10, 20, 30, 40]; // Array

    // TODO: Create a vector called `v` which contains the exact same elements as in the array `a`.
    // Use the vector macro.
    // let v = ???;

    let v = vec![10, 20, 30, 40];
    // let mut v: Vec<i32> = Vec::new();
    // for i in 0..a.len() {
    //     v.push(a[i]);
    // }

    for i in &v {
        print!("value : {} ", i);
    }
    // println!("a : {:#?}", a);
    // println!("v : {:#?}", v);
    (a, v)
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_array_and_vec_similarity() {
        let (a, v) = array_and_vec();
        assert_eq!(a, *v);
    }
}
