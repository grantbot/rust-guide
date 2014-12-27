//Basic guessing game. First run with basic Rust syntax
use std::io;
use std::rand;

fn cmp (x: uint, y: uint) -> Ordering {
    if x < y { Less }
    else if x > y { Greater }
    else { Equal }
}

fn main() {
    println!("Guess the number!");

    //Modulo 100 gives 1-99, so adding 1 gives us 1-100
    //We need uint here b/c it might gen negative ints otherwise
    //Type hint on function
    let secret_number = (rand::random::<uint>() % 100u) + 1u;

    //Initialize loop
    loop {
        println!("Input your guess!");

        //Read user input
        let input = io::stdin().read_line().ok().expect("FAILED TO READ");

        //Declaring type hint in the let. Could do from_str::<uint> without Option
        //bit. Option is an enum from std. 
        //from_str takes a &str (given by as_slice) and converts it to the type hint
        //Need trim(), else it would contain "\n" from pressing enter on input
        let input_num: Option<uint> = from_str(input.as_slice().trim());

        //Need to pattern match to extract out the fields from the Option enum
        let num = match input_num {
            Some(n) => n,
            None    => {
                println!("Please input a number");
                continue; //Don't break out of loop, just ignore input
            }
        };

        println!("You guessed {}", num);

        match cmp(num, secret_number) {
            Less => println!("Too Small"),
            Greater => println!("Too Big"),
            Equal => {
                 println!("YOU WIN");
                 //Breaks out of loop
                 return;
             },
        }
    }
}
