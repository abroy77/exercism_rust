fn main() {
    let a: [i32; 0] = [];
    let b = [1, 2, 3, 4];
    println!("{}, {}", a.len(), b.len());
    println!("{}", a.len() - b.len());
}
