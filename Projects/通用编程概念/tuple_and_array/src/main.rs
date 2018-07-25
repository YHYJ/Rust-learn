fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let one = x.2;
    println!("{}", one);


    let a = [1, 2, 3, 4, 5];

    let _first = a[0];
    let second = a[1];
    println!("{}", second);
}
