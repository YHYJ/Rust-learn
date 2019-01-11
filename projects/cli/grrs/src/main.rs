use exitfailure::ExitFailure;
use failure::ResultExt;
use structopt::StructOpt; // structopt基于clap构建，clap会自动生成--help信息

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    // 结构体
    // get user input
    /// The pattern to look for.
    pattern: String,
    /// The path to the file to read.
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

#[derive(Debug)]
struct CustomError(String);

#[allow(unused_variables)]
fn main() -> Result<(), ExitFailure> {
    // let 关键字只能在函数中使用

    // 获取结构体Cli的值
    // from_args 方法适用于main函数，提供出错时的帮助信息并立即退出程序
    let args = Cli::from_args(); // 尝试将参数解析为Cli结构

    let path = "test.txt";

    // 读取path参数指定的文件
    // read_to_string函数返回Result（包含String和某种类型的错误，在此是std::io::Error）
    let content = std::fs::read_to_string(path)
        .with_context(|_| format!("Could not read file '{}'", path))?;

    // let content = match result {
    //     Ok(content) => content,
    //     Err(error) => {
    //         // 1. 抛出错误
    //         // panic!("Can't deal with {}, just exit.", error);
    //         // 2. 返回错误信息
    //         return Err(error.into()); // return需要函数签名
    //     }
    // };

    // for line in content.lines() {
    //     if line.contains($args.pattern) {
    //         println!("{}", line)
    //     }
    // }

    println!("file content: {}", content);
    Ok(()) // 函数默认返回值
}

// ? 是 match 的快捷方式，还做了类型转换
// Box 可以包含任何实现了标准 Error 特征的类型
// 因此基本所有错误都可以放在Box里，所以可以用 ? 在所有常用函数中返回 Result
