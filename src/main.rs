fn main() {
    let x: i32 = 17;
    {
        println!("{}", x);
        let x = 12;
        println!("{}", x);
    }

    println!("{}", x);
    let x = 42;
    println!("{}", x);
}
