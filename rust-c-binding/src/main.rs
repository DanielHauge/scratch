#[link(name = "add")]
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {
    let a = 10;
    let b = 20;
    let c = unsafe { add(a, b) };
    println!("{} + {} = {}", a, b, c);
}
