fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let b = [3; 5];

    println!("{:?}", x);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{:?}", x.0);
    println!("{:?}", a[0]);
}