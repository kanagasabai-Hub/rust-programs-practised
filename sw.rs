// use std::io;
fn my_fun(number: i32){
	println!("Invalid number {}",number);
}
fn main(){
	let number = 9;
	match number {
		2 | 4 | 6 => println!("These are numbers {}",number),
		3 | 5 | 7 => println!("These are odds {}",number ),
		_ => my_fun(number),
	}
}
