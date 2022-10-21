

fn mod_inverse(x: u32, m:u32) -> () {

    if x == 0 && m == 0 {
        println!("Error bro, what do you mean?");
    }
    
    //The modulo must be positive
    // 
    if m > 0 {
     
     // assign the input parameters to mutable variables
     // for ease of reassignment and swapping
     let mut a = x;
     let mut b = m;
     

     //find the GCD(greatest common divisor) 
     //while the value of b is not negative
     let mut c1 = 1;
     let mut d1 = 0;
     let mut c2 = 0;
     let mut d2 = 1;
 
     while b > 0 {
         let q = a / b;
         let r = a % b;
 
 
 
         let c3 = c1 - q*c2;
         let d3 = d1 - q*d2;
 
         c1 = c2;
         d1 = d2;
         c2 = c3;
         d2 = d3;
         a = b;
         b = r;
 
 
     }
    
     // if the greatest common divisor is not one then 
     //there's no modular inverse for the number x and modulo m
    if a != 1 {
     println!("no modular inverse");
    }
      
    //
    let result = c1;
 
    println!("{} is the modular inverse", result);
 
 
    }else{
     println!("number must be positive");
    }
 
    
 }
 
 
 fn main(){
     mod_inverse(3,11);
 }