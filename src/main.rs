// Q-1:

//Define some variables with items names and add prices as values. Minimum 5 variables 
fn main() {
    // let lipton = 500;
    // let nido = 900;
    // let sugar = 60;
    // let red_chilli = 20;
    // let chickpeas =80;


// Q-2:
// The government updates the prices of some items kindly update the prices accordingly.

    // let nido = nido+950;
    // let sugar = sugar+80;
    // let chickpeas = chickpeas+150;


// Q-3:
// Create a function which takes a string as argument and returns its length as integer. 
  
    // let sen = String::from("Stay home...Stay safe");

    // fn check_length(s: &String)-> usize{
    //     s.len()
    // };
    // println!("{}",check_length(&sen) );




// Q: 4
// Create a function which takes a string as argument and returns its length as integer. 
   

//    let string_one = String::from("large number");
//    let string_two = String::from("Small one");

//    fn check_largest(a:&String, b:&String){
       
//     if a.len() > b.len(){
//         println!("{}",a )
//     }else{
//         println!("{}",b )
    
//     }

//    }
//    check_largest(&string_one, &string_two);

// Q: 5
// Create a function, take integers and check if the number is odd or even.

// let num = 10;
// fn even_or_odd(x:u32){
// if x % 2 == 0{
//     println!("number is even" );
// }else{
//     println!("number is odd" );
// }
// }
// even_or_odd(num);


//Q: 6

// Create 4 mathematical functions (+ - / *) with 2 arguments.

    // let x: u16 = 76;
	// let y: u32 = 23;
	// let sum = 76 + 23;
	// println!("X+Y = {}",sum);
	// let sub = 76 - 23;
	// println!("X-Y = {}",sub);
	// let mul = 76 * 23;
	// println!("X*Y = {}",mul);
	// let div = 76 / 23;
	// println!("X/Y = {}",div);

// Q: 7
// Create this pattern and print.
// *
// * *
// * * *
// * * * *        
// * * * * *

//   for n in 0..6{
//       for o in 0..n{
//           print!("*");
//       }
//       println!("");
//   }


// Q - 8:
 // Create a mark sheet to do gradings , students below 40% will be considered fail and
 // three highest percentage holders will be graded as 1st 2nd 3rd.
 let marks = 80;
 if marks >= 80{
     println!("A+ Grade");
 }else if marks >=75 && marks <= 79{
     println!("A Grade");
 }else if marks >=60 && marks <= 69{
     println!("B Grade");
 }else if marks >=50 && marks <= 59{
     println!("C Grade");
 }else if marks >=40 && marks <= 49{
     println!("D Grade");
 }else{
     println!("Fail");
 }
}