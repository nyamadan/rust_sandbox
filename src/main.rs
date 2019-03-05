fn print_number(x: i32) {
    println!("x is: {}", x);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn diverges() -> ! {
    panic!("This function never returns!");
}

fn main() {
    let f: fn(i32) -> i32 = add_one;
    print_sum(f(5), add_one(5));
    print_number(4);
    let _x: i32 = diverges();
}
