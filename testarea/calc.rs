// a calculator program using rust

use std::io;

fn p(print: String){
	println!("{}",print)
}
fn main() {
	let fval = 0;
	let sval = 0;
	let choice = 0;
	let mut input=String::new();
	p("This is a Caluclator program implemented using Rust \n".to_string());
	// print!("{:?}",fval+sval+choice );
	io::stdin().read_line(&mut input).expect("Failed to get value");
	let choice = input.trim().parse::<u32>().unwrap();
	match choice{
		1=>{p("add".to_string());},
		2=>{p("Sub".to_string());},
		3=>{p("Multi".to_string());},
		_ =>{p("Default".to_string());}
	}
	println!("Done");
}
