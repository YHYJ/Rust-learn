extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io; // std：标准库；io：输入输出功能

fn main() {
    println!("Guess the number!");

    // 生成随机数
    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        // 定义变量
        let mut guess = String::new();

        // 获得用户输入
        io::stdin()
            .read_line(&mut guess) // & 代表参数guess是一个引用以减少拷贝
            .expect("Failed to read line"); // read_line的返回值io::Result，用于错误处理；这里只是接收错误并不处理

        // String -> num
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,     // 解析成功：返回 Ok 中的数字
            Err(_) => continue, // 解析失败："_" 代表不论错误类型，continue 使 loop 进行下一次迭代
        };

        println!("You guessed: {}", guess);

        // 判断随机数和用户输入的大小
        match &guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 猜对后退出；
            }
        }
    }
}
