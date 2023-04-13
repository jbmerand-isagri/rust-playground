fn if_expression() {
    let number = 5;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn if_dans_let_statement() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}

fn loop_repetition() {
    // boucle infinie
    loop {
        println!("again!");
    }
}

fn loop_retourne_valeur() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}"); // 20
}

fn conditional_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}

fn collection_loop() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn collection_loop_2() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn for_loop() {
    for number in (1..4).rev() {
        println!("{number}");
    }
    println!("LIFTOFF!!!");
}
