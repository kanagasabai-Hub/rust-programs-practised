// just for learning
// pub struct Health{
// 	blood_group:String,
// 	medical_history:bool,
// 	is_condition:String,
// }
struct Point(u8,u8,u8);
 struct User{
	name:String,
	age:u8,
	// health:&'a Health,
	user_id:usize,
}
trait People{
	fn new(&self)->Self;
	fn inividual_factors(&self);
}
impl People for User{
	fn new(&self)->User{
		User{
			name:self.name.to_string(),
			age:self.age,
			// health:&Health{
			// 	blood_group:self.health.blood_group,
			// 	medical_history:self.health.medical_history,
			// 	is_condition:self.health.is_condition,
			// },
			user_id:self.user_id,
		}
	}
	fn inividual_factors(&self){
		if self.age >= (25 as u8){
			println!("Tender age");
		}else {
			unimplemented!();
		}
		if self.user_id == (123456 as usize){
			println!("{} is under medical prescribtion",self.name);			
		}
		else{
			println!("Good luck with the good and sound health");
		}
		println!("Current user ID:{:?}",self.user_id );
	}
}

fn main() {
	let user1 = User{
		name:"Someone".to_string(),
		age:25,
		// health:&Health{
		// 	blood_group:"B+".to_string(),
		// 	medical_history:false,
		// 	is_condition:"Good".to_string(),
		// },
		user_id:123456
	};
	let point=Point(2,5,8);
	let staff:User = People::new(&user1);
	let us = People::inividual_factors(&staff);
	println!("{:?}",us );
	// destructuring a struct
	let Point(x,y,z)=point;
	// utilising the destructured values
	println!("x:{},y:{},z:{}",x,y,z );
	// alternative method
	println!("x:{:?}, y: {:?} z: {:?}",point.0,point.1,point.2 );
}