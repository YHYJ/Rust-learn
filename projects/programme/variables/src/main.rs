fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6;  // 不能对不可变变量 x 二次赋值
    println!("The value of x is: {}", x);

    let mut y = 15;     // mut 使变量可变
    println!("The value of is: {}", y);
    y = 16;
    println!("The value of is: {}", y);

    // 一个常量
    // 命名规范：全大写、下划线分割
    const LIGHT_SPEED: u32 = 299_792_458;
    println!("The speed of light is: {} m/s", LIGHT_SPEED)
}
