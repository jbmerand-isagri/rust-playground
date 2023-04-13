// Scalar types
// integers, floating-point, number, Booleans, characters

fn integer_types() {
    let height_bit_signed: i8 = -2;
    let height_bit_unsinged: u8 = 2;

    let sixteen_bit_signed: i16 = -21212;
    let sixteen_bit_unsigned: u16 = 122123;

    // ...

    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
}

fn floating_points_types() {
    let x = 2.0; // f64 (par défaut, 64 bits signed)
    let y: f32 = 3.0; // f32 (32 bits signed)
}

fn boolean_type() {
    let t = true;
    let f: bool = false; // explicit type annotation
}

fn character_type() {
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
}

// Compound Types
// tuples and arrays

fn tuple_type() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // pattern matching pour avoir une valeur (destructuring)
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    // access a tuple element by using a period and the index of the value
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}

fn array_type() {
    // fixed length, all values are of the same types
    // useful to allocate data on the stack rather than the heap
    let a = [1, 2, 3, 4, 5];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; // same value 3, 5 times

    // access elements
    let first = a[0];
    let second = a[1];
}
