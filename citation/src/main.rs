fn main() {
    let s = String::from("LIDIFEI");

    let r1 = &s; // 没问题
    println!("{}", r1);
    let r2 = &s; // 没问题
    
    // 此位置之后 r1 和 r2 不再使用

    let r3 = &mut s; // 没问题
    println!("{},{}", r2,r3);

}
