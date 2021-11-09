// a simple program using
struct Employee{
	name:String,
	age:u8,
}
struct EmployeeDetail{
	gender:String,
	height: f64
}
trait Company{	
	fn is_age(&self);
	fn is_working(&self);
}
trait Gender{
	fn gender(gender:String,height:f64)->Self;
	fn is_gender(&self);
}
impl Company for Employee{	
	fn is_age(&self){
		if &self.age < &(25 as u8){
			println!("new employee {:?}",self.name);
			
		}
		else{
			println!("old employee {:?}",self.name);
		}
	}
	fn is_working(&self){
		if &self.age > &(40 as u8){
			println!("The expoyee {} should be relived from the company and given the extended pension",&self.name);
		}
	}

}
impl Gender for EmployeeDetail{
	fn gender(gender:String,height:f64)->EmployeeDetail{
		EmployeeDetail{gender: gender, height: height}
	}
	fn is_gender(&self){
		if &self.gender == "Male"{
			println!("Male {:?}",self.gender);
		}else{
			println!("Female");
		}
	}
}
fn main(){
	let user1=Employee{
		name:"user1".to_string(),
		age:24,
};
	let user2=Employee{
		name:"user2".to_string(),
		age:41,
};
	user1.is_age();
	user2.is_age();
	user2.is_working();
	// let det = EmployeeDetail{gender:"Male".to_string(),height:5.5};
	let det:EmployeeDetail=Gender::gender("Male".to_string(),5.5); 
	// det.gender();
	det.is_gender();
}
