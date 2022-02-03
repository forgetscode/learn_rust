use std::fmt;
//derive debug required to print on a struct
#[derive(Eq, PartialEq, Debug)] 
pub struct Point { 
    x: i32,
    y: i32,
}

fn struct_addition( a: &Point, b: &Point ) -> Point {
    let x1 = a.x + b.x;
    let y1 = a.y + b.y;
    return Point{ x:x1, y:y1};
}

// use cargo test to test
#[cfg(test)]
fn struct_addition_test() {
    assert_eq!(struct_addition(&Point{x:1,y:1}, &Point{x:1,y:1}), Point{x:2,y:2});
}


fn main() {
    let x = Point {x:1, y: 1};
    let y = Point {x:1, y: 1};
    if x == y {
        println!("match: {:#?}", x);
    }
    assert_eq!(x, y);

    let a = String::from("music");
    let b = "is nice";
    println!("match: {}", a +" "+ &(b));


}
