// an example for refrence hashmap
use std::collections::HashMap;
use std::io;

fn createHash(x: &mut HashMap<String,u64>,s1:&mut String,s2:&mut u64){	
	// using dereferencing and conversion methods
	x.insert(s1.to_string(),*s2);
}

fn main() {
	let mut mp = HashMap::new();
	let mut s:u64=1;
	let mut r:String= String::new();
	let mut ipt = String::new();
	let mut count:u8;
	loop{
	println!("Enter username or email");
	io::stdin().read_line(&mut ipt).expect("Cant read values");
	r=ipt.trim().parse().expect("Error");
	ipt.clear();
	println!("Enter the password");
	io::stdin().read_line(&mut ipt).expect("Cant read values");
	s=ipt.trim().parse::<u64>().expect("Error parsing");
	createHash(&mut mp,&mut r,&mut s);
	ipt.clear();
	println!("Do you want to continue '1' or exit '0'");	
	io::stdin().read_line(&mut ipt).expect("failed to load");
	count=ipt.trim().parse::<u8>().expect("Error parsing");
	if count == 1{ipt.clear(); continue;}
	else{break;}
	}
	println!("{:?}",mp );
}