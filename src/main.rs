fn main() {
    let mut x = 5;

    loop {
        x += x - 3;

        println!("{}", x);

        if x % 5 == 0 {
            break;
        }
    }

    for (i, j) in (5..10).enumerate() {
        println!("{}, {}", i, j);
    }
}
