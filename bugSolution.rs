fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of unsafe manipulation
    v[0] = 10;
    println!( "{:?}", v);
}