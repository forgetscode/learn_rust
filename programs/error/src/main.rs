use std::error::Error;

fn this_can_fail(succeeds: bool) -> Result<String, String> {
    if succeeds {
        Ok(String::from("Success"))
    } else {
        Err(String::from("Error"))
    }
}

fn multiple_possible_failures() -> Result<String,String> {
    this_can_fail(true)?;
    println!("After 1st potential error.");
    this_can_fail(true)?;
    println!("After 2nd potential error.");
    Ok(String::from("All done."))
}
// results must be used to compile


fn main() {
    let outcome = this_can_fail(true);
    println!("{:?}", outcome);

    match this_can_fail(true) {
        Ok(val) => println!("Success: {}", val),
        Err(err) => println!("Error: {}", err),
    }

    if this_can_fail(false).is_ok() {
        println!("It worked!");
    } else {
        println!("It didn't work!")
    }

    println!("{}", multiple_possible_failures().unwrap());

    let some_result = this_can_fail(true);
    // Only done if `some_result` is an `Ok` Variant.
    let mapped_result = some_result.map(|val| val.len());
    println!("{:?}", mapped_result);
}