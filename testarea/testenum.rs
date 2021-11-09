// struct and enum implementation
#[derive(Debug)]
enum Health{
	Good(bool),
	Normal(u32),
	Bad(bool),
}
#[derive(Debug)]
enum Gender{
	Male(u8),
	Female(u8),
	Other(u8),
}
#[derive(Debug)]
enum Age{
	Child(u8),
	Teenage(u8),
	Grown(u8),
	Old(u8),
}
#[derive(Debug)]
struct Person{
	name:String,
	age:Age,
	gender:Gender,
	health_status:Health,
}
#[allow(non_snake_case)]
trait AboutUS{
	fn how_are_you(&self);
	fn check_status(&self);
}

impl AboutUS for Person{
	fn how_are_you(&self){
		if &self.name == "Kanagu"{
			println!("{:?} is Teenage and Healthy male", self.name);
		}
	}
	fn check_status(&self){
		println!("Checking Status");
	}
}

fn main() {
	let male = Gender::Male(1);
	let female = Gender::Female(2);
	let other = Gender::Other(3);
	println!("{:#?}",male );
	let user=Person{
		name:"Kanagu".to_string(),
		age:Age::Teenage(15),
		gender:male,
		health_status:Health::Good(true),
	};
	println!("{:?}",user.health_status );
	user.how_are_you();
}