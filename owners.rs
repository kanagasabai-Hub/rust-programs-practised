//defining ownership in rust language
// the allocation and storage of memory can be done using 
// stack and heap,stack is for static un changable values that is once compiled with default value it cannot be changed
// heap is for dynamic allocations during runtime.
fn main(){
	// vector v owns the object in the heap.
	let v= vec![1,2,3];
	// the ownership of v is transfered to v1
	// if you print v, it will show error show v1 is valid
	// two pointers of the same content are not allowed in rust.
	let v1 =v;
	// println!("{:?}",v1 );
	display(v1); //v2 is moved to display fn so v2 is invalidated.
	// println!("{:?}",v1 ); //it also wont work! no longer usable
}
fn display(v1:Vec<i32>){
	println!("{:?}", v1);//ownership moved here!
}
