fn main() {
	let name = "nm";
	match name {
		"nm" => println!("{:?}","this the correct content" ),
		_ => println!("{:?}","follow the Default" ),
	}
	// println!("{}",choice );
	let bools:bool = false;
	let bin = match bools{
		true => 1,
		false => 0,
	};
	println!("{:?}",bin);
	let mut x:u8=0;
	loop {
		x+=1;
		ptn(x);
		if x==5 {
			break;
		}
	}
	pub fn ptn(x : u8){
		println!("Result: {:?}",x );
	}
	
}
