#[derive(Debug)]
struct Django {
	section:String,
	use_case:String,
	version:f64,
	advantage:{is_speed:bool,rating:f64}
}
fn main() {
	let x=5;
	let y = {
		let x=6;
		x+4
	};
	let z={|r|{r*(x+y)}};
	println!("y:{},x:{},z:{:?}",y,x,z(3) );
}