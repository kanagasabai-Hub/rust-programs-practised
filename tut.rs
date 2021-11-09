fn main(){
    let mut a:u32;
    const X:u32 = 10;
    let name = "this-is-for-splitter".split('-');
    let fullname = "testing point \r \n";
    println!("Setting done");
    let clearText = fullname.trim().to_string();
    a=0;
    for z in 1..X{        
        a+=z/2;
        if a%2 == 0 {
            println!("{}",a);
        }
        else {
                  a+=3;
                  println!("{}",a);
              }      
    }
    println!("{:b}",4>>2 );
    // for s in name{
    //     println!("name:{}",s);
    // }
    // println!("{} {}",clearText,clearText.len() );
}
