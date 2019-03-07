fn main() {
    let mut x = (1, 2);
    let y = (2, 3);

    x = y;

    let tuple = (1, 2, 3);
    let (x, y, z) = tuple;

    let t0 = tuple.0;
    let t1 = tuple.1;
    let t2 = tuple.2;
    println!("x: {}", t2);

    fn foo(x: i32) -> i32 {
        x
    }

    let x: fn(i32) -> i32 = foo;
}
