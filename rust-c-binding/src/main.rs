#[link(name = "add")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

#[link(name = "sub", kind = "static")]
extern "C" {
    fn sub(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 10;
    let b = 20;
    let c = unsafe { add(a, b) };
    println!("{} + {} = {}", a, b, c);

    let c = unsafe { sub(a, b) };
    println!("{} - {} = {}", a, b, c);
}
