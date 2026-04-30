mod vector;
fn main() {
    let a = vector::Vector{x: 5f64, y: 5f64};
    println!("{a}");
    println!("{}", a*2);
}
