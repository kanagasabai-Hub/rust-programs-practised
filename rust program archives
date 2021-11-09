// all tested rust programs archives
fn main() {
	// s in stack, value cannot be modified
	let mut ss ="hi";
	for t in ss.chars(){
		print!("{:?}",t );
	}
	// s in heap, so modification is possible
	let s = String::from("hi");
	let mut r = s;
	r.push_str(" hello");
	println!("{:?}",r);
}

//string and str operations.
fn main() {
	let v1:Vec<&str> = "Many more days to live".split(" ").collect();
	for i in v1.iter(){
		println!("{:?}",i);
	}
	let v2:Vec<String> =vec!["One more life to live".to_string();10];
	for j in v2.iter(){
		println!("{}",j );
		let divide:Vec<&str> = j.split(" ").collect();
		for n in divide.iter(){
			print!("{:?} \t",n);
			for ch in n.chars(){
				print!("{} \t",ch );
			}
			print!("\n");
		}		
		print!("\n\n");
	}
}
//string sliced

fn main() {
	let s = "hi";
	let mut t = String::from(s);
	for i in 0..5{
		t.push_str("added");
	}
	println!("{:?}",t );
	// performing string slice
	let sliced = &t[2..];
	println!("Sliced: {:?}",sliced );
}

//string to str conversion with reference borrowing and scoping
fn main() {
	let mut a = String::from("hi");

	let b = &a;
	println!("b:{:?}",b );
	let c = &a;
	println!("c:{:?}",c);
	// println!("c:{:?}",c );
	// defining mutable reference
	let d =&mut a;
	d.push_str(" here");
	println!("d:{:?}",d );
	for i in 1..100{
		let tem = i.to_string();
		// string to str conversion
		a.push_str(&tem[..]);
		a.push_str(" ");
	}
	println!("a:{:?}", a);
}
/* write a function that takes a string and returns the first word it finds in that string. 
If the function doesn’t find a space in the string, 
the whole string must be one word, so the entire string should be returned.*/
fn main() {
	let word = String::from("horaay dude");
	let result = first_word(&word);
	println!("{:?}",result );
}
fn first_word(w:&String)-> &str{
	let word_as_byte = w.as_bytes();

	for (i,&val) in word_as_byte.iter().enumerate(){
		if val == b' '{
			return &w[0..i];
		}
	}
	&w[..]
}

//dummy program
struct Users{
	name:String,
	psw:String,
	email:String,
}

trait Validator{
	fn validate(&self);
}

impl Users{
	fn disp(&self){
		println!("Username:{} , Passwords:{}",self.name,self.psw );
	}
}
impl Validator for Users{
	fn validate(&self){
		let t:Vec<&str> = self.email.split('@').collect();
		// let tail = String::from(&t[9..]);
		// println!("{:?}",t );
		if t[1] == "email.com"{
			println!("email verified");
		}
	}
}

fn main() {
	let users = Users{
		name:"kanagu".to_string(),
		psw:"somepsw".to_string(),
		email:"someone@email.com".to_string(),
	};
	users.disp();
	users.validate();
}

//a small test example
#[derive(Debug)]
enum Proof {
	aadhar,
	pan_card,
	passport,
	Birth_certificate,
	community_certifcate,
	driving_license,
	voter_id,
}
enum Person_Id{
	peron_id:String
}
enum Health{
	Good,
	Average,
	abnormal,
	as_patients,
	bad,
}
enum Workflow{
	working_status(bool),
	under_training(bool),	
}
enum Gender{
	Male,
	Female,
	Other,
}
struct Person{
	name:String,
	age: u8,
	place:String,
	gender:Gender,
	DOB:String,
	email:String,
	id_proof:Proof,
	is_working:Workflow,
	health_status:Health,

}

fn main() {
	let user1 = Person{..};
}

//tuple struct
#[derive(Debug)]
struct Color(u8,u8,u8);

fn main() {
	// tuple struct demonstration..
	let blue = Color(222,0,122);
	println!("{:?}",blue.0 );
	match blue{
		Color(x,y,z) =>{println!("x:{},y:{},z:{}",x,y,z );},
		// _ =>{},
	}
}

//this program is error
#[derive(Debug)]
struct Color;

trait Types{
	fn new()->&'static str;
	fn disp(&self);
}

impl Types for Color{
	fn new()-> &'static str{
		println!("Color is colorful..");
		"blue"
	}
	fn disp(&self){
		println!("Your choosed color: {:?}",self);
	}
}

fn main() {
	// tuple struct demonstration..
	let blue = Color::new();
	blue.disp();
}

//square and rectangle
use std::fmt;

struct Rectangle{
	width:u16,
	height:u16,
}
struct Square{
	sides:u32,
}
// display property for structs or simply use #[derive(Debug)] on the struct
impl fmt::Display for Rectangle{
	fn fmt(&self,f:&mut std::fmt::Formatter)-> fmt::Result{
		write!(f,"width: {} ,Height: {}",self.width,self.height)
	}
}

fn main() {
	let rect1 = Rectangle{
		width:20,
		height:10,
	};
	let square = Square{
		sides:30,
	};
	println!("The area of the Rectangle is: {:?}",area_rect(&rect1));
	println!("Rectangle value:{}", rect1);
	println!("The area of the square is : {:?}",area_square(&square) );
}

fn area_rect(rect1:&Rectangle)->u16{
	rect1.width * rect1.height
}

fn area_square(sq:&Square)->u32{
	sq.sides*sq.sides
}

//factorial problems

fn main() {
	let mut num =5;
	let factorial =fact(num);
	println!("{:?}",factorial );
	let fact = fct(&mut num);
	// the reason the num value is not update is 
	// the called function did not return the num value instead
	// it returns the called functions scoped value.. 
	println!("{:?}, num: {:?}",fact,num );
	let res = recur(&mut num);
	println!("Function return value:{:?},\nmain function val:{:?}",res,num );
	println!("Factorial with reference:{:?},\nmain function: {:?}",fact_ref(&mut num),num );
}

fn fct(y:&mut u32)->u32{
	let mut f = 1;
	for j in 1..*y+1{
		f*=j as u32;
	}
	f
}

fn fact(x:u32)->u32{
	let mut factorial:u32 =1;
	for i in 1..x+1{
		factorial =factorial*i as u32;
	}
	factorial
}

// using recursion
fn recur(num:&mut u32)->u32{
	// let mut n:u32 =0
	if *num == 1{
		*num
	}else{
		*num * recur(&mut (*num -1))
	}	
}

fn fact_ref(num:&mut u32)->u32{
	let mut fact:u32 =1;
	 for i in 1..*num+1{
	 	fact *= i;
	 }
	 *num =fact;
	 *num
}

// enum and struct
#[allow(dead_code)]

#[derive(Debug)]
enum Ipkind {
	V4,
	V6,
}

struct Ipaddr{
	kind:Ipkind,
	address:String,
}

impl Ipaddr{
	fn new(&self)->Ipaddr{
		Ipaddr{
			kind:match self.kind{
				Ipkind::V4 =>Ipkind::V4,
				Ipkind::V6 =>Ipkind::V6,
			},
			address:self.address.to_string(),
		}
	}
	fn disp(&self,xt:&str){
		println!("Network Type: {:?} ,\nIP Address: {} \n{}",self.kind,self.address,xt );
	}
}

fn main() {
	let conn = Ipaddr{
		kind:Ipkind::V4,
		address:String::from("127.0.0.1")
	};
	// println!("{:#?}",conn.kind );
	conn.new();
	conn.disp("thats it");
}

//test options type
fn test_option(val:&i8)->Option<i8>{
	if *val > 5 as i8{
		Some(*val)
	}else{
		None
	}
}

fn main() {
	let x:i8=10;
	let y:Option<i8>=test_option(&x);

	let sum = match y{
		Some(r)=> x+r,
		None => {0},
	};
	println!("{:?}",sum );
}

//coin finder
#[derive(Debug)]
enum Usestate{
	Pdkt,
	Trichy,
	Chen,
	Regular,
}

#[derive(Debug)]
enum Coins{
	Small,
	Medium,
	Broad(Usestate),
}

fn coin_finder(coin: Coins)->u8{
	match coin{
		Coins::Small => 1,
		Coins::Medium => 2,
		Coins::Broad(state) =>{println!("This coin has {:?} state",state);5},
	}
}


fn main() {
	let my_coin =coin_finder(Coins::Broad(Usestate::Pdkt));
	println!("Value of the coin: {:?}",my_coin );
}

// roll a dice and if let

fn main() {
	let roll =9;
	match roll{
		3=>add_hat(),
		5=>remove_hat(),
		1=>gain(),
		_=>reroll(),
	}
	let some:Option<i32> = Some(10);
	if let Some(x)=some{
		println!("If ok x{}",x );
	}else{
		println!("not found dude");
	}
}

fn add_hat(){println!("added +1");}
fn remove_hat(){println!("removed -1");}
fn gain(){println!("gained 'added' +1");}
fn reroll(){println!("reroll");}

//mod testing
#[allow(non_snake_case)]
pub mod Things{
	pub mod Are{
		pub fn working(){
			println!("yes its working");
			nested_one();			
		}
		fn nested_one(){
			println!("this is a nested_one");
			super::things();
			super::private_things();
		}
		pub mod addtional{
			pub fn core_things(){
				crate::Things::core_thing();
			}
		}
	}
	fn things(){
		println!("the outer things");
		private_things();
	}
	fn private_things(){
		println!("This is the outer private space");
	}
	fn core_thing(){
		println!("from base to core call");
	super::outside_mod();
		
	}
}

fn outside_mod(){
	println!("A call from the outside fn to inside mod");
}

fn main() {
	Things::Are::working();
	Things::Are::addtional::core_things();
}

//palindrome
fn main() {
	let pal = String::from("color");
	let mut tem =String::new();
	for i in pal.chars().rev(){
		tem.push(i);
	}
	assert_eq!(pal,tem);
	println!("palindrome");	
}

//rust lifetime testing
fn main() {
	let a =String::from("abc");
	let b =String::from("absc");
	// here result is a primitive type 'str' which implement copy trait
	// so no ownership here..
	let result = greater(&a.as_str(),&b.as_str());
	println!("{}",result );
	let c = result;
	println!("second one:{:?}",c );
	println!("{:?}",result );
}

fn greater<'a>(p1:&'a str,p2:&'a str)->&'a str{
	if p1.len()>p2.len(){
		println!("a is greater");
		p1
	}else{
		p2
	}
}

//a one step for good understanding of rust lifetime

/ this program will cause error if you remove the owner variable 
// i have used ownership so no error if not, the 
// error will happen inside the scope of variable b.
fn main() {
	let a =String::from("abc");
	let result: &str;
	let owner:String;
	// the lifetime of a variable depends on the scope of the variable
	// let it be ref or owner both will have the same scope
	// incase of multiple refrence param for a function
	// then the lifetime of the returned value will be the lifetime that has
	//smallest scope of the one of ref parameter.
	{
		let b =String::from("absc");
		result = greater(&a.as_str(),&b.as_str());
		println!("inside small scope:{}",result );
		owner = String::from(result);
	}
	// let c = result;
	let c = owner;
	println!("outer scope:{:?}",c );
}

fn greater<'a>(p1:&'a str,p2:&'a str)->&'a str{
	if p1.len()>p2.len(){
		println!("a is greater");
		p1
	}else{
		p2
	}
}
//tricky lifetime
fn main() {
	let mut x=1;
	let y:&i32=f(&mut x);
	println!("y:{}",y);
	println!("{:?}",x );
}

fn f(x:&mut i32)->&i32{
	if *x > 0{
		*x=(*x)+1;
		x
	}else{
		&0
	}
}

// use cargo rand package from crates.io in order to use this program
use rand::Rng;
use std::io;
fn main() {
    let mut generator = rand::thread_rng();
    let mut input=String::new();
    let mut count:i32=0;
    loop {
        if count > 100{
            println!("Better luck next Time...🤭");
            break;}
        let gen = generator.gen_range(0..100);
        println!("Counter:[{}] Guess your number within 100..can you? [Enter the guessed number]> ",count);
        io::stdin().read_line(&mut input).expect("cant read the input");
        let guessed = input.trim().parse::<i32>().expect("Cant parse the value");
         input.clear();
         let rng_guessed = if guessed>100{
            println!("\nEnter within the range! [system guess:{}]\n",gen);
            continue;
        }else{
            guessed
        };
        count+=1;
        if gen == rng_guessed{
            println!("Hola! Your guess is right \nuser:{}\nsystem:{}",guessed,gen );
            break;
        }else{
            println!("\n[system guess:{}]> Reguess..",gen);
            // rng_guessed = 0;       
            continue;
        }

    }

}

//ascednding and descending

fn main() {
	let mut dat:Vec<i32>=vec![12,45,776,33,55,1];
	for i in 0..dat.len(){
		for j in i+1..dat.len(){
		if dat[i] > dat[j]{
			let temp = dat[i];
			dat[i]=dat[j];
			dat[j]=temp;
		}
	  }
	}
	println!("ascending order:{:?}",dat );

	for i in 0..dat.len(){
		for j in i+1..dat.len(){
		if dat[i] < dat[j]{
			let temp = dat[i];
			dat[i]=dat[j];
			dat[j]=temp;
		}
	  }
	}
	println!("descending order: {:?}",dat );
}

// search a value in vec
fn main() {
	let mut dat:Vec<i32>=vec![12,45,776,33,55,1];
	let srch:i32 = 33;
	for (index,value) in dat.iter().enumerate(){
		if *value == srch{
			println!("the location of {} in vec:{:?} is in index :{}, value:{}",srch,dat,index,dat[index] );
		} 	 
	}
}

// search a value in vec
fn main() {
	let mut dat:Vec<&str>=vec!["hi","hello","how is","well fine"];
	let srch:&str = "hello";
	for (index,value) in dat.iter().enumerate(){
		if *value == srch{
			println!("the location of {} in vec:{:?} is in index :{}, value:{}",srch,dat,index,dat[index] );
		} 	 
	}
}

//trait and trait objects

struct Rectangle{
	width:i32,
	height:i32,
}
struct Sphere{
	x:i32,
	y:i32,
	z:i32,
}

trait Area{
	fn area(&self)->i32;
}

trait ExtArea:Area{
	fn extarea(&self){
		println!("This is extra area:{:?}",self);
	}
}

impl Area for Rectangle{
	fn area(&self)->i32{
		self.width*self.height
	}	
}

impl Area for Sphere{
	fn area(&self)->i32{
		self.x*self.y*self.z
	}
}


// trait bounding using trait as object ie: trait object
fn area<T: Area>(t:&T)->i32{
	t.area()
}
fn main() {
	let rect = Rectangle{
		width:45,
		height:20,
	};
	let spre=Sphere{
		x:2,
		y:33,
		z:4
	};
	println!("Area of the Rectangle:{:?}",area(&rect));
	println!("Some area of the Sphere:{:?}",area(&spre) );
}

//using closure with Fn operator

use std::ops::Fn;

// fn f(x:i32)->i32{x}

// the F notation represents Fn type 
fn calculate_cube<F>(f:F)->i32 
	where F:Fn(i32)->i32 {
		f(3)  //closure call with value 3
}

fn main() {
	// a normal closure
	let multiple = |x|x*x;
	println!("Simple multipel:{:?}",multiple(5) );

	// closure using Fn operator
	let cube = |y|{y*y*y};
	let result = calculate_cube(cube);
	println!("Cube value:{:?}",result );
}

// concatenate the string using + symbol
// the + symbol signature is fn add(&self,s:&str)-> String{} 
// that's why we use second string as refrenced.
// s2 will be moved to s3 due to ownership
fn main() {
	let s1=String::from("first");
	let s2=String::from("second");
	let s3=s1 + &s2;
	println!("{}",s3 );
	//alternative way without ownership
	let result = format!("{}{}",s1,s2);
	//the above code wont work because of s1 ownership is moved so no more access of s1.
}

// hashmap usage

use std::collections::HashMap;

fn main() {
	let team =vec![String::from("csk"),String::from("knr")];
	let score = vec![98,68];
	// one method to insert data inside hashmap
	let mut stats:HashMap<_,_> =team.into_iter().zip(score.into_iter()).collect();
	println!("{:?}",stats );
	// another way
	stats.insert("kxiip".to_string(),88);
	println!("{:?}",stats );
	// search through hashmap and the search result will be option type
	match stats.get("csk"){
		Some(x)=>{println!("Searched value:{:?}",x );},
		None =>{println!("No data matched");}
	}
	// iterate through hashmap
	for (key,value) in &stats{
		println!("{}: {}",key,value );
	}
	// inserting value if key has no value
	stats.entry(String::from("rcb".to_string())).or_insert(60);
	println!("{:?}",stats );
	// updating the value based on the old value
	let txt = "hello this is me and me";
	let mut mp = HashMap::new();
	for word in txt.split_whitespace(){
		let count = mp.entry(word).or_insert(0);
		*count +=1;
		/*
		The or_insert method actually returns a mutable reference (&mut V) to the value for this key.
		 Here we store that mutable reference in the count variable, 
		 so in order to assign to that value, 
		 we must first dereference count using the asterisk (*). 
		 The mutable reference goes out of scope at the end of the for loop, 
		 so all of these changes are safe and allowed by the borrowing rules. 
		*/
	}
	println!("{:?}",mp );
}

// hashmap usage 1

use std::collections::HashMap;

fn main() {
	let values:Vec<i32> = vec![3,54,2,6,9,2,6];
	let mut temp:i32=0;
	for i in 0..values.len() as i32{
		temp+=values[i as usize];
	}
	let res = temp/values.len() as i32;
	println!("mean: {:?}",res );
}

// mean median mode

use std::collections::HashMap;

fn main() {
	// mean median and mode
	let mut values:Vec<i32> = vec![3,54,2,6,9,2,6,89,9,22,9];
	let mut temp:i32=0;
	let range = values.len() as i32;
	for i in 0..values.len() as i32{
		temp+=values[i as usize];
	}
	let res = temp/values.len() as i32;
	println!("mean: {:?} range:{}",res,range );
	values.sort();
	// values.reverse();
	// println!("{:?}", values);
	let mid = range/2;
	let median = values[mid as usize];
	println!("median: {:?}",median );
	let mut count=0;
	let mut mode:HashMap<usize,i32>=HashMap::new();
	for i in 0..range-1{
		// for j in i+1..range{
			let j:usize =(i+1) as usize;
			if values[i as usize] == values[j]{
				count+=1;
				mode.insert(count,values[i as usize]);
				// println!("Duplicate value:{:?}",values[i as usize] );
			// }
		}
	}
	println!("Mode(values occured most often) calculation in the given vector:{:?}",values );
	for (key,_val) in mode.iter().enumerate(){
		let tp = key+1;
		match mode.get(&tp){
			Some(x)=>{println!("mode:{} \t",x );},
			None =>{println!("no value here..");}
		}
	}
	println!("total number of modes :{}",count);

}

// 
// mean median mode updated

use std::collections::HashMap;

fn main() {
	// mean median and mode
	let mut values:Vec<i32> = vec![3,54,2,6,9,2,6,89,9,22,9];
	let mut temp:i32=0;
	let range = values.len() as i32;
	for i in 0..values.len() as i32{
		temp+=values[i as usize];
	}
	let res = temp/values.len() as i32;
	println!("mean: {:?} range:{}",res,range );
	values.sort();
	// values.reverse();
	// println!("{:?}", values);
	let mid = range/2;
	let median = values[mid as usize];
	println!("median: {:?}",median );
	let mut count=0;
	let mut mode:HashMap<usize,i32>=HashMap::new();
	for i in 0..range-1{
		// for j in i+1..range{
			let j:usize =(i+1) as usize;
			if values[i as usize] == values[j]{
				count+=1;
				mode.insert(count,values[i as usize]);
				// println!("Duplicate value:{:?}",values[i as usize] );
			// }
		}
	}
	println!("Mode(values occured most often) calculation in the given vector:{:?}",values );
	for (key,_val) in mode.iter().enumerate(){
		let tp = key+1;
		match mode.get(&tp){
			Some(x)=>{println!("mode:{} \t",x );},
			None =>{println!("no value here..");}
		}
	}
	println!("total number of modes :{}",count);
	
}

// rust threading
// use std::fs::File;
use std::thread;
use std::time::Duration;
fn main(){
    // let file = File::open("doc.txt").unwrap();
    let v= vec![1,2,3,5,6,7,8];
    let t1=thread::spawn(move ||{
        for i in v.iter(){
        println!("this is thread {}",i);
        thread::sleep(Duration::from_secs(1));
        }
    });
    for j in 0..10{
    println!("hi from the main thread {}",j);
    thread::sleep(Duration::from_secs(1));
    }
    // joining the main thread to the sub thread
    // if i move the t1.join line to above the main thread it'll first
    // compile the subthread first and then the main thread. 
    t1.join().unwrap(); 
}

//error handling
fn main() {
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");

        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut s = String::new();

        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    println!("result:{:?}", read_username_from_file());
}

//error handling 1
use std::fs::File;
use std::io::Read;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut s = String::new();
    let _file = File::open("file.txt")?.read_to_string(&mut s)?;

    println!("{:?}", s);
    Ok(())
}

//time based program
//use chrono external crate from crate.io in order to run this program
// get the current date and the time
use chrono::prelude::*;
fn main() {
    let utc = Utc::now();
    let local = Local::now();
    // let zone: DateTime = zone(Local::now());
    println!("utc:{:?}\nlocal{:?}\n", utc, local);
}


//find the largest number in an array or vec
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
}

//generic
#[derive(Debug)]
struct Gen_fn<T>(T, T);
fn main() {
    // create blog
    // pending review
    // authenticated
    // live
    let x1 = 10;
    let x2 = 20;
    let generic = Gen_fn(x1, x2);
    println!("{:?}", generic);
    let chr = Gen_fn("dfs", "afds");
    println!("{:?}", chr);
}