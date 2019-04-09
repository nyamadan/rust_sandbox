fn sum_len(v1: &Vec<i32>, v2: &Vec<i32>) -> usize {
    v1.len() + v2.len()
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![1, 2, 3];

    let sum = sum_len(&v1, &v2);

    println!("v1.len: {}", v1.len());
    println!("v2.len: {}", v2.len());
    println!("sum_len: {}", sum);
}
