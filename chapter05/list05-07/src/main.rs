fn main() {
    let n: f32 = 42.42;
    let n_bits = n.to_bits();
    let sign_bit = n_bits >> 31;

    println!("{}", sign_bit);
}
