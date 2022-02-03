
use std::fmt::Formatter;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

struct DotBad {
    x: i32,
    y: i32
}

#[derive( Debug, Clone, Copy)]
struct Dot {
    x: i32,
    y: i32
}

fn pacman(dot: Dot) {
    println!("Eating {:?}", dot);
}

fn pacmanbad(dot: DotBad) {
    println!("Eating {}", dot);
}

fn inspect(p: &DotBad) { 
    println!("{}", p);
}

fn inspectPoint(p: &Point) { 
    println!("{:?}", p);
}

impl std::fmt::Display for DotBad {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        writeln!(f, "x gay: {}, y gay : {}", self.x, self.y )
    }
}

fn move_point(
    p: &mut Point,  
    x: i32, y: i32
) {
    p.x = x;
    p.y = y;
}

fn drop<T> (_: T) {
}


fn main() {
    let dot = Dot { x: 1, y: 2 };
    pacman(dot);
    pacman(dot);
    let _dot = DotBad { x: 1, y: 2 };
    //pacmanbad(_dot); 2 times does not work because copy and clone traits not implemented so it gets consumed
    pacmanbad(_dot);


    let mut point = DotBad { x: 1, y: 2 };
    let re = &point; 
    //point.x = 2;  Won't compile because you cannot alter something that's being referenced
    inspect(re);   
    point.x = 2;   

    let mut p = Point { x: 1, y: 2 };
    inspectPoint(&p); 
    move_point(&mut p, 3, 3); 
    inspectPoint(&p);

    let pi = 12;
    //imutable, can have multiple immutable borrows
    let b1 = &pi;
    let b2 = &pi;
    // and multiple mutable borrows
    let mut b3:&i32 = &pi;
    let mut b4:&i32 = &pi;

    let mut pm = 13;
    //mutable, can have multiple immutable borrows
    let b5 = &mut pm;
    let b6 = &mut pm;

    // but not multiple mutable borrows, b8 invalid
    let mut b7 = &mut pm;
    //let mut b8 = &mut pm; 

    drop(b7);
    //deallocates b7 immediately

    let vec = vec![1,2,3];
    let iter = vec.iter();
    drop(iter);
    println!("{:?}", vec);

}

