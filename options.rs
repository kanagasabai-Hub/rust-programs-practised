// Type Option represents an optional value: 
// every Option is either Some and contains a value, or None, and does not.

fn divide(top:f64,btm:f64)->Option<f64>{
	if btm == 0.0{
		None
	}
	else{
		Some(top/btm)
	}
}

fn main() {
	let numr = 25.0;
	let deno = 5.0;
	let deno1 = 0.0;
	let res = divide(numr,deno);
	let res = divide(numr,deno1);
	match res {
		Some(val) => println!("Divided: {:?}",val as u8 ),
		None => println!("Divided by Zero"),
	}
}