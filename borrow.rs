// ownership will move the data to the new owner assignment
// once moved then the data of previous var will result in error
//  to override that Borrowing is used 
// it gives the ownership temporatily to the assiginig variable
// using the '&' symbol, once the execution of temp var ot
// function is done the ownership is regained as it is 
// so you can access the value after that..
// Example
fn main(){
	let mut v = vec![20,430,54];
	brwfn(&mut v);
	println!("in main function: {:?}",v );
}
fn brwfn(v: &mut Vec<i32>){
	v[0] = 25;
	println!("outside function: {:?}",v );
}