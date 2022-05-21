fn main() {
    //Interger
    let a: i32 = 10; //Decimal
    let b: i32 = 0xff; //Hex
    let c: i32 = 0o77; //Octal
    let d: i32 = 0b1111_1111; //Binary
    let e: u8 = b'A'; //Byte(u8 only)
    let f: u8 = 255; //u8 maximo de 255, 256 em diante reinicia a contagem, 256=0, 257=1
    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
    println!("{}", e);
    println!("{}", f);

    float();
    booleans();
    characters();

    compound_types();
    let sum: i32 = my_function(11, 22);
    println!("The sum is:{}", sum);

    control_flow();
}

fn float() {
    //Floationg-point numbers
    let g: f64 = 2.0;
    let h: f64 = 3.0;
    println!("{}", g);
    println!("{}", h);
    //addition
    let sum: i32 = 5 + 10;
    //subtraction
    let difference: f64 = 95.5 - 4.3;
    //multiplication
    let product: i32 = 4 * 30;
    //division
    let quotient: f64 = 56.7 / 32.2;
    //remainder
    let remainder: i32 = 43 % 5;

    println!("{}", sum);
    println!("{}", difference);
    println!("{}", product);
    println!("{}", quotient);
    println!("{}", remainder);
}
fn booleans() {
    //Booleans
    let t: bool = true;
    let f: bool = false;
    println!("{}", t);
    println!("{}", f);
}

fn characters() {
    //Characters
    let c: char = 'z';
    let z: char = 'Z';
    let heart_eyed_cat: char = '😻';
    println!("{}", c);
    println!("{}", z);
    println!("{}", heart_eyed_cat);
}
fn compound_types() {
    let tup: (&str, i32) = ("Bruno Costa", 16);
    let (_nome, _idade) = tup;
    let nome = tup.0;
    let idade = tup.1;

    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];

    let byte = [0; 8];

    println!("{:?}", &tup);
    println!("{:?}", &nome);
    println!("{:?}", &idade);
    println!("{:?}", &not_found);
    println!("{:?}", &byte);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y
}

fn control_flow() {
    let number = 5;

    if number < 10 {
        println!("first condition was true");
    } else if number < 10 {
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);

    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter;
        }
    };
    println!("The result is {}", result);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        println!("{}!", number);
    }
}
