fn main() {
   let a: i32 = 36;
   let b: f64 = 3.2;
   let c: char = 'g';
   let d: [i32; 5] = [2, 3, 4, 5, 6];
   let g: [i32; 1] = [8];
   let tup: (i128, f64) = (65, 63.1);
   let tum: (i32, i16 ,i8) = (52, 12, 5);

   let x = tup.0;
   let y = tup.1;
   //let z = tup.2;

   let d1 = d[0];
   let d2 = d[1];
   let d3 = d[2];
   let d4 = d[3];
   let d5 = d[4];
   let a = 1;
   let fa = fuc_name();
   //let fa = 5;

  // let (x, y, z) = tum;

//   println!("{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", a, b, c, d, g, tup, tum, x, y, d1, d2, d3



    let s = 5;

    let y = {
        let x = 3;

        x + 1 
    };
  

    let h = {
        let j = 5;

        j + 4
    
    };

}


fn fuc_name() -> i32 {
    5
}



