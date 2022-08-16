use rand::Rng;
use std::cmp::Ordering; //ordering simply an enum that is the result to being two things compared.
                        //which returns either the thing is lesser , equal to or greater.
use std::io; //to take users input we use this library

fn main() {
    println!("Guess in number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number will be : {}", secret_number);

    loop {
        println!("Please input your Guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to read the line.");

        let guess: u32 = guess.trim().parse().expect("Pleae type number!");

        println!("You gussed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("To Small."),
            Ordering::Greater => println!("To Big."),
            Ordering::Equal => {
                println!("You Win.");
                break;
            }
        }
    }
}

/////////////////// Extra Information //////////////////

//std in will give us a handle to the statnderd input.
//read_line => will the take the user input and append it to the specifire buffer in this case is our guess string.
//read line => returns result type which is an enumeration

// enum Result<T, E>{
//     Ok(T),
//     Err(E),
// }

//trim => will remove any wide space at the beginning or end of the line
//parse => will conver our string into intiger

//thread_rng() => give us a random number generator
//gen_range(1, 101) => will actually generate random number between 1 to 101.
