// A map is a collection of key-value pairs (called entries). 
// No two entries in a map can have the same key. 
// In short, a map is a lookup table.
// A HashMap stores the keys and values in a hash table.
// The entries are stored in an arbitrary order. 
// The key is used to search for values in the HashMap. 
// The HashMap structure is defined in the std::collections module.
// This module should be explicitly imported to access the HashMap structure.

use std::collections::HashMap;

fn main(){
	// creating hashmap instance
    let mut hashmp = HashMap::new();
    // insert values in hashmap using insert keyword
    hashmp.insert("this is key", "this is value");
    hashmp.insert("key1", "value1");
    hashmp.insert("key2", "value2");
    hashmp.insert("key3", "value3");

    // print the len of hashmp
    println!("{:?}",hashmp.len());
    // get() Returns a reference to the value corresponding to the key..
    match hashmp.get(&"key2"){
        Some(value) => {println!("Exist:{:?}",value)},
        None =>{ println!("Found nothing")},
    }
    // iter() Returns an iterator containing reference to all key-value pairs in an arbitrary order.
    for (key,value) in hashmp.iter(){
        println!("key: {:?} , value: {:?}",key,value)
    }
    if hashmp.contains_key(&"key1"){
        print!("Key exist \n");
    }
    // remove() removes key from the map
    hashmp.remove(&"key2");
    println!("{:?}",hashmp);

}