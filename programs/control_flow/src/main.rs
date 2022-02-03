use std::path::Path;
use std::fs::File;
use std::io;
use std::io::Read;

//if statement

fn if_control( a:i32, b:i32 ) -> bool {
    if a == b {
        return true;
    }
    else{
        return false;
    }
}

//match statement

fn match_control( a:u32, b:u32 ) -> bool {
    match a % b {
        0 => true,
        _ => false,
    }
}


// for loop
fn for_loop_control(a:&[i32]) -> i32 {
    let mut i = 0;
    for each in a {
        i+=1;
    }
    return i
}

//loop

fn loop_control (a:&i32) -> i32 {
    let mut b = *a;
    let i = *a + 5;

    loop {
        b += 1;
        if b == i {
            return b
        }
    }
}

//while 

fn while_control (a:&i32) -> std::vec::IntoIter<i32> {
    let mut i = 0;
    let mut ret = vec![];
    while i != *a {
        i+=1;
        ret.push(i);
    }
    let mut ret = ret.into_iter();
    return ret;
}

//early return 

fn early_return_control(path: &std::path::Path) -> Result<String, io::Error> {
    let mut f = File::open(path)?;

    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    return Ok(buffer);

}


fn main() {
    println!("if control: {}", if_control( 1,1 ));
    println!("match control: {}", match_control( 1,1 ));
    let a = vec![1,2,3];
    println!("for loop control: {}", for_loop_control(&a));
    let arg = 6;
    println!("loop control: {}", loop_control(&arg));
    println!("while control: {:?}",while_control(&arg));

    println!("while control: {:?}",early_return_control(Path::new("../../notes/types.txt")));
}