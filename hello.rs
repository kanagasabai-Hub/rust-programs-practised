
fn main(){
    println!("Rust says Hello!");
    let company_string = "TutorialsPoint";  // string type
    let rating_float = 4.5;                 // float type
    let is_growing_boolean = true;          // boolean type
    let icon_char = '♥';                    //unicode character type
    let result = 10;    // i32 by default
    let age:u32 = 20;
    let sum:i32 = 5-15;
    let mark:isize = 10;
    let count:usize = 30;
    //let interest:f32=10;
    let special_character = '@'; //default
    let alphabet:char = 'A';
    let emoji:char = '😁';

    println!("company name is:{}",company_string);
    println!("company rating on 5 is:{}",rating_float);
    println!("company is growing :{}",is_growing_boolean);
    println!("company icon is:{}",icon_char);
    println!("result value is {}",result);
    println!("sum is {} and age is {}",sum,age);
    println!("mark is {} and count is {}",mark,count);
    //println!("checking values {}",interest);
    println!("special character is {}",special_character);
    println!("alphabet is {}",alphabet);
    println!("emoji is {}",emoji);
}
