use colored::*;
use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Adivinhe um numero!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("O numero secreto é: {}", secret_number);
    loop {
        println!("Insira um numero");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Você adivinhou: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Menor".red()),
            Ordering::Greater => println!("{}", "Maior".purple()),
            Ordering::Equal => {
                println!("{}","Você acertou!".green());
                break;
            }
        }
    }
}
