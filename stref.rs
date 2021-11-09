// mutating the string reference
fn main(){
	let mut value:String = String::from("this is tesing");
	adder(&mut value);
	println!("After modification: {:?}",value );
}
fn adder(value: &mut String ){
	println!("inside ext fun {:?}", value);
	value.push_str(" using rust!");
}