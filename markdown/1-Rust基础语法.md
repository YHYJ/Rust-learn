# Rust基础语法

---

## 导入库

```rust
use std::io;    		// 从标准库 std 导入输入输出库 io ；
use std::cmp::Ordering	// Ordering是一个枚举类型

extern crate rand;		// 调用crate rand ；
use rand::Rng;			// trait(特性) Rng 定义了随机数生成器应实现的方法
```

---

## 定义函数

```rust
fn main() {}
// fn 语法声明了一个函数；
// () 表示没有参数；
// {} 里是函数体
```

---

## 打印

```rust
println!("Guess the number!");
// println! 是一个在标准输出上打印字符串的 宏 ；
```

---

## rand (随机数)

```rust
// rand::thread_rng() 函数提供实际使用的随机数生成器：位于当前执行线程本地，并从操作系统获取 seed ；
// 随机数生成器的 gen_range() 方法由 rand::Rng 定义，生成在两个参数范围间的随机数；
// 未指定 "secret_num" 的类型，默认 "i32"：一个32位整型；
let secret_num = rand::thread_rng().gen_range(1, 101);
```

---

## 变量及其mut

```rust
let mut guess = String::new();
// let 语句创建 guess 变量来存储用户输入；
// Rust中变量默认不可变，mut 使一个变量可变；
// String::new() 返回一个 String 的新实例作为 guess 的绑定值；
// String 是标准库提供的字符串类型，是UTF-8编码的可增长文本块；
```

---

## 关联函数

```rust
let mut guess = String::new();
// :: 表明 new() 是 String 类型的一个 关联函数(associated function) ；
// 关联函数 (静态方法) 针对类型实现，new() 在此是与 String 数据类型而非它的特定实例关联；
```

---

##实例

```rust
let mut guess = String::new();
// new() 创建了一个新的空 String ；
// new() 惯例用来创建类型实例；
```

---

## 获取输入

```rust
// 调用 io 的关联函数 stdin() ；
//  io::stdin() 返回一个 std::io::stdin() 的实例，是一个指向终端标准输入的句柄；
//  read_line() 从标准输入句柄获取用户输入；
// 无论用户输入什么，read_line() 都将其存入到一个字符串，因此需要一个字符串参数；
// 这个字符串参数需要是可变的，以便 read_line() 将用户输入附加上去；
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

---

## 引用

```rust
//  & 表示这个参数是一个 引用(reference) ，它允许多初代码访问同一内存块的数据（减少拷贝）；
// 在此引用了 guess 变量；
// 引用是一个复杂的特性，Rust的一个主要优势就是安全而简单的操作引用；
// 引用默认不可变，因此需要 mut 使其可变；
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

---

## expect

```rust
// 在此 read_line() 返回一个枚举 io::Result；
//  Result 枚举的作用是编码错误处理信息，成员是 Ok 或 Err ；
// Result类型使用它的方法 expect 来处理，当返回 Result::Err 时， expect会使程序崩溃并显示 Err 信息 + 自定义附带信息，；如果返回 Result::Ok ， expect 会获取 Ok 中的值并返回，在此是用户输入的字符串的字节数；
io::stdin().read_line(&mut guess).expect("Failed to read line");
```

---

## 字符串格式化和数字类型

```rust
// 第二个 guess 是用户输入的str；
// String的 trim() 方法去掉其开头结尾的空白；
// String的 parse() 方法将其解析为指定类型的数字；guess: u32 指定数字的类型
// i32：32位整型；u32：无符号32位整型；i64：64位整型（Rust有一系列内建数字类型）
let guess: u32 = match guess.trim().parse() {
  Ok(num) => num,     // 解析成功：
  Err(_) => continue, // 解析失败："_" 代表不论错误类型，continue 使 loop 进行下一次迭代
};
```

---

## 枚举

```rust
//  Result 类型是枚举，通常也写作 enums ；
// 枚举类型是一个固定值的集合，即枚举的 成员 ；
// match 获得 parse() 方法的 io::Result 这个枚举；
//  Result 枚举的成员是 Ok 或 Err ；
// Ok 表示操作成功，内部包含代表成功的值；Err 表示操作失败，包含失败的前因后果；
let guess: u32 = match guess.trim().parse() {
  Ok(num) => num,     // 解析成功：？num 是什么
  Err(_) => continue, // 解析失败："_" 代表不管错误类型，continue 使 loop 进行下一次迭代
};

// 枚举 Ordering 的成员是 Less、Greater、Equal (比较两个值可能出现的3种结果)
// cmp() 获取 guess 和 secret_num 的值的引用然后比较，返回一个Ordering；
// 例如 guess=50，secret_num=38,50 > 38，cmp 方法返回 Ordering:Greater
match guess.cmp(&secret_num) {
  Ordering::Less      => println!("Too small!"),
  Ordering::Greater   => println!("Too big!"),
  Ordering::Equal     => {
    println!("You win!");
    break;    // 猜对后退出；
  }
}
```

---

## match

```rust
// match 表达式根据对 guess 和 secret_num 调用 cmp 返回的 Ordering 来决定接下来的操作；
// 一个 match 的表达式由 分支(arms) 构成，一个分支包含一个 模式(pattern) 和表达式开头的值与分支模式相匹配时应该执行的代码；
// Rust获取提供给 match 的值并挨个检查每个分支的模式；
// 例如上面 “枚举” 模块的例子， match 获得 Ordering::Greater 并与分支的模式匹配；
match guess.cmp(&secret_num) {
  Ordering::Less      => println!("Too small!"),
  Ordering::Greater   => println!("Too big!"),
  Ordering::Equal     => {
    println!("You win!");
    break;    // 猜对后退出；
  }
}
```



---

## 循环

```rust
loop {}
```