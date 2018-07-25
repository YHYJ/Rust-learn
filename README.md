# Rust-learn

---

## Rust的组件

* rustc

  > 编译器
  >
  > ```shell
  > rustc main.rs
  > ```
  >
  > 会生成一个可运行的二进制文件`main`

* cargo

  > - 构建系统
  > - 包管理工具
  >
  > ```shell
  > cargo new project_name		# 创建一个新项目
  > cargo build					# 构建项目
  > cargo run					# 构建并运行项目
  > cargo build --release		# 优化编译的项目
  >
  > cargo doc --open			# 构建所有本地依赖提供的文档
  > ```
  >
  > 1. `cargo new project_name --bin`使`Cargo`快速创建一个项目骨架
  >
  >    > `--bin`参数使`Cargo`创建一个可执行程序而非一个库
  >
  > 2. `cargo build`生成的二进制文件路径是`/project_name/target/debug/**``
  >
  >    > ``**` 取决于`Cargo.toml[package][name]`
  >
  > 3. 在运行了`cargo build`以后运行`cargo run`并不会重新构建项目，Cargo发现文件并没有改变，所以只是运行了生成的位二进制文件；只有在修改了文件后`cargo run`才会重新构建项目并运行
  >
  >
  > 4. 对于简单的项目，`cargo`并不比使用`rustc`要好多少，不过在开始使用`crate`时就会显出差异
  >
  > > 对于包含多个`crate`的项目，让`cargo`来协调构建将会轻松很多
  > >
  > > > `crate`即其他语言中的“库 (library)”或“包 (package)”
  > >
  > > 在`Cargo.toml[dependencies]`下写依赖项并指定版本，语法是：
  > >
  > > ```toml
  > > rand = "0.3.0"
  > > # Cargo理解语义化版本，"0.3.0" 实际上是 ^0.3.0 的简写，代表“任何兼容0.3.0的版本”；rand = "=0.3.0" 精确指定版本号；也可以指定版本范围；
  > > ```
  > >
  > > 更多`Cargo`用法参考
  > >
  > > ```shell
  > > cargo --help
  > > ```
  > >
  > > 和[The Cargo Book](https://doc.rust-lang.org/cargo/)
  >
  > ​	

* rustup

---

## Cargo

> `Cargo`是rust的构建系统和包管理工具，可以**使用Cargo来管理Rust项目**

### Cargo负责三个工作：

1. 构建代码
2. 下载代码依赖库
3. 编译`依赖库(crate)`

### Cargo期望的目录结构：

> `Cargo`期望源文件位于`src`目录
>
> 将项目顶级目录（例如`Projects/Genesis`）留给 README、license 信息和其他跟代码无关的文件

* project_name		# 项目名称(Genesis)
  * Cargo.toml	 # `cargo`的配置文件

  * Cargo.lock        # 编译后生成，确保构建是可重现的，除非手动升级`crate`

    > 可以使用`cargo update`手动升级，但这样不会跨版本升级，例如：
    >
    > 当前使用的`rand`版本是`0.3.12`，现在有两个新版本`0.3.15`和`0.4.0`，`cargo update`只会将`rand`升级到`0.3.15`，只有修改`Cargo.toml`指定`rand = "0.4.0"`才会升级到`0.4.0`

  * README.md、license信息和其他跟代码无关的文件
    * src	         # 源文件目录
      * main.rs  # 主函数（固定名字）
    * lib.rs          # 库文件（固定名字）

