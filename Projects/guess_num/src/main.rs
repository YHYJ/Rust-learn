extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");

    // rand::thread_rng() 函数获取一个随机数生成器的拷贝；
    // rand::thread_rng() 等效于在开头 use rand::thread_rng; ；
    // gen_range() 方法隶属于 rand::Rng ；
    // 未指定 "secret_num" 的类型，默认 "i32"：一个32位数；
    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("Please input your guess.");

    // let 语句创建变量绑定；默认不可变；mut 使一个绑定可变；
    // String 是一个字符串类型，由标准库提供，是一个可增长的UTF-8编码的文本；
    // :: 语法：创建一个“关联函数”；与 String 而非它的特定实例自身关联；类似“静态方法”；
    // new() 关联到 String，创建一个新的空的 String；new() 通常用来创建一个类型的新值；
    // Rust判断 "guess" 是 "String" 类型
    let mut guess = String::new();

    // io::stdin() 返回一个指向终端标准输入的句柄；
    // read_line() 用这个句柄获取用户输入，传递给参数 &mut guess；
    // Rust 的“引用”功能，允许对一个数据有多个引用以减少拷贝；
    // 引用默认不可变，因此需要 &mut guess 使其可变而非 &guess；
    // 在此 read_line() 返回一个 io::Result；
    // 这个 Result 类型的作用是编码错误处理信息；
    // except() 方法获取 io::Result 的值
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {}", guess);

    println!("The secret number is: {}", secret_num);

    // cmp() 比较 'guess' 和 'secret_num'，并获取比较的值的引用；返回一个 "Ordering" ；"Ordering" 是一个枚举；
    // "match" 获取 "Ordering" 的值，并为每一种值创建一个”分支“；
    // "Ordering" 将”分支“分别匹配 "Less / Greater / Equal" 并交给相应的处理语句
    match guess.cmp(&secret_num) {
        Ordering::Less      => println!("Too small!"),
        Ordering::Greater   => println!("Too big!"),
        Ordering::Equal     => println!("You win!"),
    }
}
