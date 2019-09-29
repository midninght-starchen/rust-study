fn main() {
    let number = 100;


    if number % 52 == 0 {
    
        println!("number is divisible by 52");

    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } 


    let ssd = 63;

    if ssd > 50 {
        let s = uk(3, 8);
        fn uk(s: i32, j: i32) ->  i32 {
           s + j 
        }
        println!("{}", uk(3, 8));
    }



    let sss = 54;
    
    if sss > 50{
        let y = jjj(5, 9);
        fn jjj(h: i32, p: i32 ) ->  (i32) {    
        
          (h * p)
        }
    
        println!("{}", y);   
    }


    //let condition = ture;

   // let number = if condition {
    
     //   5
   // } else {
      //  "six"
    //};

  //  println!("the value of number is: {}", number);



    //loop {
       // println!("ag");
    //}




    let mut counter = 0;

    let result = loop {
        counter = counter + 1;

        if counter == 10 {
            break counter * 2;
        } 
    };

    println!("{}", result);

    assert_eq!(result, 20);

    let mut number = 3;

    while number != 0 {
    
        println!("{}!", number);

        number = number - 1;
    }
    println!("liftoff!!!");


    let a = [10, 20, 30, 40, 50];

    let mut index = 0;

    while index < 5 {
    
        println!("the vaule is: {}", a[index]);

        index += 1;

    }

    for element in a.iter() {
        println!("{}", element);   
    }

    let a1 = 1 ;
    let a2 =100;
    for x in a1..a2 {
        println!("{}", x);
    }

}







