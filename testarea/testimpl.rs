// implementing a struct and impl

struct Person{
	name:String,
	age:u8
}
struct User{
	id:String,
	age:u8,
}
trait Character{
	fn new(name:String,age:u8)->Self;
	fn defined(&self);
}

impl Character for Person{
	fn new(name:String,age:u8)->Person{
		Person{
			name:name,
			age:age
		}
	}
	fn defined(&self){
		if &self.age > &(20 as u8){
			println!("{} is elder",self.name );
		}else{
			println!("{} is younger", self.name );
		}
	}
}
fn is_counter(x:u8)->bool{
	if x <=5{
		true
	}else{
		false
	}
}
fn main() {
	let user:Person = Character::new("This User".to_string(),20);
	user.defined();
	let mut i = 0;
	// testing some while function
	while is_counter(i){
		println!("its raining {}",i );
		i+=1;
	}
	// refrencing a struct in a trait..copy trait
	let Person{name,age} = &*&user;
	println!("{:?}",age );
}