// a simple implementation of loop and its function using rust
fn main(){
    let mut count =0;
    'first_loop: loop{
        let mut rem = 5;
        loop{            
            // print!("{}",rem);
            if rem == 3{
                break;
            }
            if count == 2{
                println!("Exciting the inner loop!");
                break 'first_loop;
            }
            println!("rem:{}, count:{}",rem,count);
            rem -=1;
        }
        count +=1;
        println!("count outer:{}", count);
    }
    println!("Exciting the outer loop!");
    println!("returning values from the loop:");
    let mut counter =0;
    let result = loop {
        if counter == 10{
            break counter*2;
        }
        counter+=1;
    };
    println!("final result:{}",result );
    println!("Looping through the collection:");
    let a = [1,2,3,4,5];
    let mut id = 0;
    while id < 5{
        println!("array loop val: {:?}", a[id]);
        id +=1;
    }
    println!("looping through the collection using for..");
    // wise choice for iterations
    for num in a{
        println!("using for loop: {:?}",num );
    }
    println!("for loop with rev function");
    for iter in (1..5).rev(){
        println!("{:?}",iter );
    }    
}