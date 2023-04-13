use rand::Rng;
// le set de bibliothèques standards s'appelle "the prelude"
use std::cmp::Ordering;
// standard library :: input/output library
use std::io;

fn main() {
    //guess();
    array_test();
}

fn guess() {
    // println! est une macro (grâce au !)
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        // on déclare une variable mutable
        // :: indique que new est une fonction du type String
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // & indique que l'argument est une référence
            .expect("Failed to read line");
        // read_line retourne un Result
        // si l'instance de Result est Err, expect va faire crasher le programme et afficher le message

        // shadow de la précédente valeur de guess (utile pour changer le type)
        let guess: u32 = match guess.trim().parse() {
            // match expression
            Ok(num) => num,     // first match arm
            Err(_) => continue, // permet d'ignorer l'erreur
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            // match expression (ends after the first successful match)
            Ordering::Less => println!("Too small!"), // one arm pattern
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // quitte la boucle
            }
        }
    }
}

fn space() {
    let spaces = "   "; // string type
    let spaces = spaces.len(); // number type
}

fn array_test() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    // runtime error si index > 4 ; Rust quitte immédiatement
    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
