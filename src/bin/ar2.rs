
fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn display_result(result: i32) {
    println!("The result is: {}", result);
}

fn main() {
    let a = 10;
    let b = 20;
    let result = sum(a, b);
    display_result(result);
}

// cargo run -q --bin ar2