
// this struct generic will have the same type for all vars inside it
struct Point<T> {
    p1: T,
    p2: T,
}
// this struct generic will have different type for different var inside it
// example: x can be i32 and y can be f64,or chars, or Strings, vice versa..
#[derive(Debug)]
struct Points<T, U> {
    x: T,
    y: U,
}

// implementing Point as type T like Point<T>
// so one have to say that Point<T> implements type T which is generic ie:any type
// thats why you have write it as impl<T>
impl<T: Copy + std::ops::Add<Output = T> + std::fmt::Debug> Point<T> {
    // add code here
    fn new(&self) -> Point<T> {
        Point {
            p1: self.p1,
            p2: self.p2,
        }
    }
    fn out(&self) {
        println!("Same type vars in generics: {:?}", self.p1 + self.p2);
    }
}

// here Point struct has f32(concrete types) type defined so
// one can implement Point type f32 without telling the impl like impl<f32>
// it is predefined so no type specification in impl.
impl Point<f32> {
    fn non_gen(&self) -> f32 {
        self.p2
    }
}

impl<T: Copy + std::fmt::Debug, U: Copy + std::fmt::Debug> Points<T, U> {
    fn new(&self) -> Points<T, U> {
        Points {
            x: self.x,
            y: self.y,
        }
    }
    fn out(&self) {
        println!("Different vars types in generics:{:?},{:?}", self.x, self.y);
    }
}

fn ots<T: std::fmt::Debug>(t: &Point<T>) {
    println!("{:?}", t.p1);
}

fn main() {
    let r = Point { p1: 3.5, p2: 44.0 };
    r.new();
    r.out();
    ots(&r);
    let res = r.non_gen();
    println!("{:?}", res);
    let gen1 = Points { x: 34, y: 6.7 };
    gen1.new();
    gen1.out();
}
