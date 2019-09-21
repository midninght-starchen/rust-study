fn main() {
    let x: i32 = 5;
    let x: i32 = x + 1;
    let x: i32 = x * 2;
    //let a: i32 = x.len();
    //println!("{}", a);
    println!("The vuale of x is: {}", x);
    let spaces = "      ";
    let spaces = spaces.len();
    println!("{}",spaces);

    const A: i32  = -12345;
    println!("{}",A);

    let fx = 3.70;  //f64
    let fy = 4.90; // f32
    let add = fx + fy;
    
    println!("{} + {} = {}", fx, fy, add);
    // addition
    let sum = 5 + 10;
    
    // suntraction
    let diffrence = 99.5 -4.3;

    //multiplication
    let product = 4 *30;

    //division
    let quotient = 56.7 / 32.2;

    //remainder 
    let remainder = 43 % 5;

    println!("{} {} {} {} {} ",sum, diffrence, product, quotient, remainder);
    
    let t = true;

    let f: bool = false; // with explicit ytpe annotation

    let c = 'z';
    let z = 'z';
    let heart_eye_cat = '*';
    println!(" {} {} {} {} {}",t, f, c, z, heart_eye_cat );

    let tup: (i8, i16, i32, i64,f32, f64, u8, u16, u32, u64, char) = (1, 2, 3, 4, 1.11111, 5.222222, 5, 4, 6, 7, 'a');

    println!("{:?}", tup);


    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is : {}", y);


}
