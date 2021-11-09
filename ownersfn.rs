// using ownership returning a value form the function
fn main(){
	let v = vec![5,6,7];//vector v owns the object in heap
	let v2=v; //moved ownership to v2
	let result = disp(v2);
	println!("in main: {:?}",result );
}

fn disp(v:Vec<i32>)->Vec<i32>{
	println!("outside function: {:?}", v);
	// accessing the vec values
	for i in 0..3{
		println!("{:?}",v[i] );
	}
	return v;
}