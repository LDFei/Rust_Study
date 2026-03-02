use std::io;
use rand::Rng;

fn main() {
    println!("猜数字游戏！");
    
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop 
    {
    println!("请输入你要猜的数字：");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("读取输入失败！");
    
    let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => 
            {
                println!("请输入一个有效的数字！");
                continue;
            }
        };
    
    println!("你猜的为： {}", guess);
        if guess < secret_number {
            println!("太小了！");
        } else if guess > secret_number {
            println!("太大了！");
        } else {
            println!("恭喜你，猜对了！");
            break;
        }
    }
}