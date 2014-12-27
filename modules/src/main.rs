fn main() {
    //Call function from module
    hello::print_hello();
}

mod hello {
    //Need pub so things outside of the mod can use it. This is 'exporting' in
    //Rust
    pub fn print_hello() {
        println!("Hello World!");
    }
}
