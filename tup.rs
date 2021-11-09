fn main(){
	// creating a tuple with the mutable values
	let mut tp:(u32,f64,u8,bool)=(349,55.5,99,true);
	println!("before changing tup value: {:?}",tp);
	tp.0 = 999;
	println!("changed tup value: {} and all tup value: {:?}",tp.0,tp );
	print(tp);
}
fn print(x:(u32,f64,u8,bool)){
	// assigining a tuple to a distinct variable
	let (rate,m1,m2,is_passed)=x;
	println!("Rate:{:?} \n m1: {:?} \n m2: {:?} \n is_passed: {:?}",rate,m1,m2,is_passed );
}