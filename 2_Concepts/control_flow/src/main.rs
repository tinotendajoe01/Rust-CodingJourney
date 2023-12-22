fn main (){
    let number = 3;
     if number <5{
        println!("Condition was true");
     }else {
        println!("Condition was false");
     }
     branching();
   count_down()
   }

fn branching (){
     let number = 6;
     if number % 4 == 0 {
        println!("number is divisible by 4");
     }else if number % 3 == 0 {
        println!("number is divisible by 3");
     }else if number % 2 == 0 {
        println!("number is divisible by 2");
     } else {
        println!("number is not divisible by 4,3 or 2");
     }
}

fn count_down (){
   for number in  (1..5).rev(){
      println!("{number}!");
   }
   println!("LIFTOFF!!!");
}