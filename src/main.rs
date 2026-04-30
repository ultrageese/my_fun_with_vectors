mod vector;
fn main() {
    let a = vector::Vector{x: 5f64, y: 5f64};
    println!("{a}");
    let b =vector::Vector{x:3.0, y: 2.0};
    println!("{}", a*b);
}
