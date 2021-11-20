fn main() {
    let a: f32 = 42.42;
    let frankentype: u32 = unsafe {
        std::mem::transmute(a)
    };

    println!("{}", frankentype);
    // `std::fmt::Binary`トレイトによる2進数の整形で、左から32桁は0埋めせよ、という意味
    println!("{:032b}", frankentype);

    let b: f32 = unsafe {
        std::mem::transmute(frankentype)
    };

    println!("{}", b);
    assert_eq!(a, b);
}
