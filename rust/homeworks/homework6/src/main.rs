fn main() {
    println!("Running a fizzBuzz()...");
    fizz_buzz();
}

fn fizz_buzz(){

    for i in 1..=301 {
        if i%3 == 0 && i%5 == 0 {
            println!("FizzBuzz");
        }
        else if i%3 == 0 {
            println!("Fizz");
        }
        else if i%5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", i)
        }

    }
}