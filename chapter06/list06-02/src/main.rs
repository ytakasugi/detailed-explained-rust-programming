use std::mem::size_of;
use std::mem::size_of_val;
fn main() {
    let n: u8 = 25;
    let n_ptr: *const u8 = &n;
    let m: Box<u8> = Box::new(25);
    let array: [u8; 10] = [99, 97, 114, 114, 121, 116, 111, 119, 101, 108];
    let slice = &array;
    let slice_box: Box<[u8]> = Box::new(array);
    let v: Vec<u8> = vec![99, 97, 114, 114, 121, 116, 111, 119, 101, 108]; 
    let s = "Alice".to_string();
    let str = "Alice";
    
    // プリミティブ型
    println!("n: {}", n);
    println!("Memory Location: {:p}", &n);
    println!("size_of::<u8>(): {} byte", size_of::<u8>());
    println!("size_of_val(&n): {} byte", size_of_val(&n));

    println!("{}", "---------------------------------------------------------------");

    println!("n_ptr: {:?}", n_ptr);
    println!("Memory Location: {:p}", &n_ptr);
    println!("size_of::<*const u8>(): {} byte", size_of::<*const u8>());
    println!("size_of_val(&n_ptr): {} byte", size_of_val(&n_ptr));

    println!("{}", "---------------------------------------------------------------");

    println!("m: {}", m);
    println!("Memory Location: {:p}", m);
    println!("size_of::<Box<u8>>(): {} byte", size_of::<Box<u8>>());
    println!("size_of_val(&m): {} byte", size_of_val(&m));

    println!("{}", "---------------------------------------------------------------");

    // 配列
    println!("array: {:?}", array);
    println!("length: {}", array.len());
    println!("Memory Location: {:p}", &array);
    println!("size_of::<[u8; 10]>(): {} byte", size_of::<[u8; 10]>());
    println!("size_of_val(&array): {} byte", size_of_val(&array));

    println!("{}", "---------------------------------------------------------------");

    // スライス
    println!("slice: {:?}", slice);
    println!("length: {}", slice.len());
    println!("Memory Location: {:p}", &slice);
    println!("referent: {:p}", slice);
    println!("size_of::<&[u8]>(): {} byte", size_of::<&[u8]>());
    println!("size_of_val(slice): {} byte", size_of_val(slice));
    println!("size_of_val(&slice): {} byte", size_of_val(&slice));

    println!("{}", "---------------------------------------------------------------");

    // Box化されたスライス
    println!("slice_box: {:?}", slice_box);
    println!("length: {}", slice_box.len());
    println!("Memory Location: {:p}", &slice_box);
    println!("referent: {:p}", slice_box);
    println!("size_of::<Box<[u8]>>(): {} byte", size_of::<Box<[u8]>>());
    println!("size_of_val(&slice_box): {} byte", size_of_val(&slice_box));

    println!("{}", "---------------------------------------------------------------");

    // ベクタ
    println!("v: {:?}", v);
    println!("length: {}", v.len());
    println!("capacity: {}", v.capacity());
    println!("Memory Location: {:p}", &v);
    println!("size_of::<Vec<u8>>(): {} byte", size_of::<Vec<u8>>());
    println!("size_of_val(&v): {} byte", size_of_val(&v));

    println!("{}", "---------------------------------------------------------------");

    // String
    println!("s: {:?}", s);
    println!("length: {}", s.len());
    println!("capacity: {}", s.capacity());
    println!("Memory Location: {:p}", &s);
    println!("size_of::<String>(): {} byte", size_of::<String>());
    println!("size_of_val(&s): {} byte", size_of_val(&s));

    println!("{}", "---------------------------------------------------------------");

    // 文字列スライス
    println!("str: {:?}", str);
    println!("length: {}", str.len());
    println!("Memory Location: {:p}", &str);
    println!("referent: {:p}", str);
    println!("size_of::<&str>(): {} byte", size_of::<&str>());
    println!("size_of_val(str): {} byte", size_of_val(str));
    println!("size_of_val(&str): {} byte", size_of_val(&str));

}
