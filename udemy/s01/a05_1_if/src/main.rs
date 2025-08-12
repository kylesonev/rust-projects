use std::io;

fn main() {
    println!("Digite um número: ");
    let mut entrada = String::new();

    io::stdin()
        .read_line(&mut entrada)
        .expect("Falha ao ler entrada");

    let number: i32 = entrada.trim().parse().expect("Digite um número válido!");

    if number < 5 {
        println!("O número é menor que 5.");
    } else if number == 5 {
        println!("O número é 5!");
    } else {
        println!("O número é maior que 5");
    }
}
