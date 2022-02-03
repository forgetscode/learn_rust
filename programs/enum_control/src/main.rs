use std::fs::File;
use std::io;

//enums with match

enum Direction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
}

fn enum_match_control( dir: &Direction ) -> bool {
    match dir {
        Direction::West( _ ) => true,
        _ => false,
    }
}


//important enums, already defined inside rust, T is a generic
/*
enum Option<T> {
    Some(T),
    None,
}
*/
//possible absence of value
/*
enum Result<T,E> {
    Ok(T),
    Err(E),
}
*/
//might return error


fn main() {
    // pass in pointer to object to a function otherwise function deallocates on scope
    println!("enum control: {}", enum_match_control( &Direction::West(2)));

    //option example
    let will_overflow: Option<u8> = 10_u8.checked_add(250);
    match 10_u8.checked_add(250) {
        Some(sum) => println!("interesting: {}", sum),
        None => eprintln!("addition overflow!"),
    }
    //result example
    let file_open: Result<File, io::Error> = File::open("Does not exist");

    match file_open {
        Ok(f)  => println!("Success!"),
        Err(e) => println!("Open failed: {:?}", e),
    }

    let sum: Option<u8> = Some(6);

    match sum {
        Some(sum) if sum % 2 == 0 => println!("5+5 is even!"),
        _ => println!("5+5 ... isn't even?"),
    }
}