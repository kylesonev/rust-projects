extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Advinhe o número!");
    println!("Será gerado um número de 0 a 10!");
    let numero_gerado = rand::rng().random_range(1..=10);
    println!("---------------------------");
    loop {
        println!("Digite o seu palpite: ");

        let mut palpite = String::new();

        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        let palpite: i32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Digite um número válido!");
                continue;
            }
        };

        println!("Você disse: {}", palpite);

        match palpite.cmp(&numero_gerado) {
            Ordering::Less => println!("Palpite muito baixo!"),
            Ordering::Greater => println!("Palpite muito alto!"),
            Ordering::Equal => {
                println!("Parabéns! Você acertou!");
                break;
            }
        }
    }
}
