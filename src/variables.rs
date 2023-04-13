// Constante
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

// Shadowing (utilisation de let à chaque fois)
// permet de ne pas avoir à donner différents noms
// et de changer de type
fn function1() {
    let x = 5;

    let x = x + 1; // 6

    // inner shadowing
    {
        let x = x * 2; // 12
        println!("The value of x in the inner scoope is: {x}");
    }

    println!("The value of x is : {x}"); // 6

    let guess: u32 = "42".parse().expect("Not a number!");
}
