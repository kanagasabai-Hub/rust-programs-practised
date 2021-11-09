// dereferencing model for box type or smart pointer
#[derive(Debug)]
struct Abox<T>(T);
struct Bx<T>{
	a:T,
}
impl<T> Abox<T>{
	fn new(a:T)->Abox<T>{
		Abox(a)
	}
}
// a == *b wont work above, to make it work we use Dref trait
use std::ops::Deref;
impl<T> Deref for Bx<T>{
	type Target = T;
	fn deref(&self)->&T{
		&self.a
	}
}

// implementing Drop
// the drop will deallocate the allocated memory when it goes out of scope
// in simple the drop activates when the var goes no use in program
// it just work as destructor in cpp
impl<T> Drop for Bx<T>{
	fn drop(&mut self){
		println!("values droped!");
	}
}

fn main() {
	let a =5;	
	let b = Abox::new(a);
	println!("{:?}",b);
	let c  = Bx{a:5};
	println!("{:?}",*c.deref() );
	if a == *c.deref(){
		println!("True");
	}
}