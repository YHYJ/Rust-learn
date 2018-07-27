fn main() {
    println!("Hello, world!");

    another_function(5);     // 只要定义了一个函数，不管它在main函数的何方都可以调用

    statement_expression();
}

fn another_function(x: i32) {     // Rust代码采用 ’snake case‘ 命名风格：所有字母都是小写，以下划线分隔单词
    println!("The value of x is: {}", x);
}

fn statement_expression() {     // 语句和表达式
    let se = {                  // {} 是一个表达式
        let x = 3;
        x + 1
    };

    println!("The value of se is: {}", se);
}
