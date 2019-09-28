fn main() {

    //zhuerba_function();

    //another_function(32);
    //sum_function(2, 3);
   // double_function(75, 7, 5);
    println!("{}", div_function(5571.2, 3.5));
    println!("{}", model_function(10, 3));
    println!("{:?}", xx_function(5, 6));



}


   
fn zhuerba_function() {
    println!("zhuerba_function.");

}

    
fn another_function(x: i32) {
    println!("the value of  x is: {}", x);

}

fn sum_function(a: i32, b: i32) {
    println!("{}", a + b);
}

fn double_function(a: i32, b: i32, c: i32) {
    println!("{}", a * b * c)
}

fn div_function(a: f64, b: f64) -> f64  {
    
    a / b
    
      
}

fn model_function(a: i32, b: i32) -> i32 {

    a % b
}

fn xx_function(a: i32, b: i32) -> (i32, i32){
    (a, b)
}
