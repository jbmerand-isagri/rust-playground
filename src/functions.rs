fn one_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn print_value() {
    let y = {
        // scope block qui est une expression
        let x = 3;
        x + 1 // ne pas mettre ; car c'est une expression
              // avec un ; serait alors une déclaration qui ne retournerait pas la valeur
    };
    println!("The value of y is: {y}"); // 4
}

// retourne_valeur_implicitement
fn five() -> i32 {
    5 // dernière expression retournée implicitement
}
