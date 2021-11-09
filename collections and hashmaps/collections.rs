// collection library provides efficient implementations
// of the most common general-purpose prog'ng 
// data structures,
// commonly used collections are : Vector,HashMap and HashSet
// here vector is implemented
// a vector is an resizable array,and flexible in runtime
// mem of vec is allocated in heap

fn main(){
	#[allow(unused_mut)]
	// using vec! macro
	let mut vect = vec![1,2,3,4,5];
	// creating new instance of vec
	let mut vect1 = Vec::new();
	for i in 0..5{
		vect1.push((i+i) as u16);
	}
	// particular values can be accessed using vect[index_num]
	// or vext val can be accessed through referencing to the vec collection variable
	// and then iterate to get individual value
	for z in &vect{
		println!("individual value:{:?}",z );
	}
	println!("vec macro:{:?},\nvec instance:{:?}",vect,vect1 );
	// implementing len, remove, contain
	// new and push are done above
	println!("vect len: {:?},\nvect1 len:{:?}",vect.len(),vect1.len() );
	if vect.contains(&5) == true{
		println!("value exists \n");
	}
	// specify the index of the vec to remove the value
	vect1.remove(2);
	println!("final vec instance: {:?}",vect1 );

}