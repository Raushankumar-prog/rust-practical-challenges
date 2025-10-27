
unsafe extern "C" {
    fn add(a: i32, b: i32) -> i32;
}

fn main() {

    let a = 10;
    let b = 20;

    let result = unsafe { add(a, b) };

    println!("add({}, {}) = {}", a, b, result);
}
