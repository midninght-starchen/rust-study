fn main() {
    let a: i32 = 63;
    let b: char = 'm';
    let c: f32 = 3.2;
    let tup = (32, 25.3, 'g');
    let (x, y, z) = tup;
    let f =[1, 2, 5, 6];
    println!("{}, {}, {}, {:?}, {}, {}, {},{:?}", a, b, c, tup, x, y, z, f);
    model1(10, 3);
    
    println!("{}",model2(100, 30));

    println!("{}", xii(32, 85));
}

fn model1(a:i32, b:i32) {
   let c =  a % b;
   println!("{}", c);
}


fn model2(d: i32, f: i32)  -> i32 {
   d % f 
} 

fn xii(b: i32, k: i32)  -> (i32) {
    (b + k )
}
