// a simple concept to understand 
// ownership and borrowing

fn main() {
	let mut x = 10;
	println!("x:{:?}",x );
	let mut y = *&x;
	y +=10;
	println!("y:{:?}",y );
	y+=10;	
	println!("x:{},y:{}",x,y);
	let z = &mut x;
	*z+=10;
	// we cannot print both mutable and immutable variable at the same time
	println!("z:{:?},y:{}",z,y);
	*z+=10;
	println!("z:{:?}",z );
	// here x becomes an immutable variable
	// including that you cannot use z var outside the scope of x as it may lead to 
	// compile error, the lifetime of z should be within the lifetime of x!
	// if you try to use z var after x, it cause error as immutable borrow..
	{
	x+=10;}
	// th below code will raise error
	// if x == *z{
	// 	println!("True");
	// }
	println!("z:{:?}",x);

}