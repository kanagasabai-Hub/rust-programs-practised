use std::io;
use std::io::{Read,Write};
use std::fs::File;
// use console::Emoji;

// pub static CRAB: &str="🦀";

fn main() {
	let mut inpt=String::new();
	let val=input(&mut inpt);
	println!("{:?}",val );
	// closure declaration, it can act as function with single
	// logic return statement
	let p=|val|{val*val};
	println!("{}", p(val));
	let cval:String = p(val).to_string();
	let mut content=String::new();
	let mut file = File::create("doc.txt").expect("unable to open the file");
	file.write_all(cval.as_bytes()).expect("write failed");
	// read file
	let mut f1=File::open("doc.txt").expect("unable to open");
	f1.read_to_string(&mut content).unwrap();
	println!("{:?}",content );
	// println!("{:?}",CRAB );
}
fn input(x:&mut String)->u8{
	io::stdin().read_line( x).expect("cannot read IO");
	return x.trim().parse::<u8>().expect("failed");
}