

fn main() {
    println!("Hello, world!");
   apply_to_jobs(5, "rust developer"); 
  let result =  is_even(7);
  println!("The number is : {result}");
  let tup_result = alphabets("zthisaaa");
  println!("the tuple result is : {tup_result:?}");
}

fn apply_to_jobs(number: i32, title: &str){
    println!("I am applying to {number} {title} jobs");
}

fn is_even(number: i32) -> bool{
    if number%2 ==0{
        return true;
    }return false;
}

fn alphabets (text: &str ) -> (bool,bool){

    ((text.contains("a")),(text.contains("z")))

}
