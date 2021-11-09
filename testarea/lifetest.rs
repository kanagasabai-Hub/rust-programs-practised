// rust lifetime test

fn main() {
	let a=20;
	let f;
	{
		let b=30;
		f=b + a;
		println!("{}",&b );
	}
	println!("{}",f);
}