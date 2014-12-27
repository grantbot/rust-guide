//extern crate testing;  //Not sure what's going on here

//Only compile this when cfg(test) is false.
#[cfg(not(test))]
fn main() {
    println!("Hello, world!");
}
