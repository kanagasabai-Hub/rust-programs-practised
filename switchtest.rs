use std::io;
fn main(){
    println!("Enter the choice to perform math operations \n");
    println!("1.Add,2.Sub,3.Mul,4.Div,5.Modulo \n");
    let mut num=String::new();
    io::stdin().read_line(&mut num).expect("Failed to load");
    println!("{}",num);
    let r=num;
    match r.trim().parse(){
        "text" => println!("Some text 1"),
        "tex"=> println!("Some text 2"),
        "te"=> println!("Some text 3"),
        "texts"=> println!("Some text 4"),
        _=> {println!("\n Enter within the choice!");},

        };
}
