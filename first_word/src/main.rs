use std::io;
fn main() {
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("读取输入失败！");

    let first_word = tool(a.trim());
    println!("输入的字符串的第一个单词为： {}", first_word);
}
fn tool(a: &str) -> &str {
    
    if let Some(index) = a.find(' ') {
    &a[..index]
    } else {
    a
}
}
