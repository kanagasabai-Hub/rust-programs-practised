// slices are similar to arrays and their length is not known
//in their compile time,
// it is a two-word object first is pointer second is len of slice
// slices can be used to borrow a section of an array
// can be denoted as slice: &[varType],example

use std::mem;

fn main(){
	// intialise an array with 1
	let mut xs:[u16;50]=[1;50];
	let mut i:u8;
	for z in 1..50{
		xs[z]= (z+z) as u16;
	}
	println!("Size of the array in bytes: {:?} \n",mem::size_of_val(&xs));
	i=0 as u8;
	// borrow a whole array as slice
	check_slice(&xs,i);
	
	i+=1 as u8;
	// borrow a section of slice
	check_slice(&xs[5..15],i);

	// alternatively inside main function
	let c=&xs[20..35];
	println!("Inside main function: {:?}",c );
}

fn check_slice(slice: &[u16],i:u8){	
	println!("{:?} \n",slice);
	// print the length of slice
	println!("Length of Slice {}: {:?} \n \n",i,slice.len() );	
}
