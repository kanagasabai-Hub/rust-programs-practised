// HashSet is a set of unique values of type T.
 // Adding and removing values is fast, 
 // and it is fast to ask whether a given value is in the set or not. 
 // The HashSet structure is defined in the std::collections module.

 use std::collections::HashSet;

 fn main(){
 	let mut sets = HashSet::new();
    let mut s = HashSet::new();
    s.insert("this works");
    let val:[&str;5] = ["a","b","c","d","e"];
 	for i in 0..5{
 		let mut x="set".to_string();
 		x.push_str(val[i]);
 		sets.insert(x.to_string());

 	}
 	println!("HashSets: {:?}",sets );

    // if (assert_eq!(sets.get(&"seta" as &str),true )){
    //     println!("Exist");
    // }
    match sets.get(&"seta" as &str){
      Some(value) =>{println!("Matched: {:?}",value);},
      None=> println!("There is nothing"),
    }

    sets.remove(&"seta" as &str);
    assert!(s.contains(&"this works" as &str));
    println!("Final sets: {:?},{:?}",sets,sets.len() );
    sets.clear();
    println!("Cleared sets: {:?},{:?}",sets,sets.len() );
 }