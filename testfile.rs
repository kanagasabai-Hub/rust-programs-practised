fn main()
{
    let name1="Hello Me,Yoo!".to_string();
    let num:u32=21;
    println!("Hi {}",name1);
    println!("value {}",num);
    for x in 1..num{
        println!("x:{}",x);
    }
    println!("{:b}",!num );
}
