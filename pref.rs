fn main(){
	let mut a:usize = 50;
	pas_by_ref(&mut a);
	println!("{:?}",a );
}
pub fn pas_by_ref(a: &mut usize){
	*a=1;
	// for l in 2..6{
	// 	// used dereferencing
	// 	*a=l-1;
	// 	if *a > 10 && *a <= 0
	// 	{
	// 		break;
	// 	}
	// 	pas_by_ref(&mut(*a - 1));
	// }
}