use rand::Rng; // Importa a trait Rng para gerar um número aleatório
use std::cmp::Ordering; // Importa enum Ordering (Less, Greater e Equal)
use std::io; // Importa o módulo de entrada/saída padrão

fn main() {
    println!("Advinhe o número!");
    println!("Será gerado um número de 0 a 10!");

    // Geração do número aleatório usando o Rng com alcance de 1 a 10
    let numero_gerado = rand::rng().random_range(1..=10);
    println!("---------------------------");

    loop {
        println!("Digite o seu palpite: ");
        // let declara a variável e mut permite que ela seja mutável
        let mut palpite = String::new();

        // io::stdin() retorna a entrada padrão (teclado)
        // .read_line(&mut palpite) lê uma linha e armazena na variável
        // &mut passa uma **referência mutável** da string
        io::stdin()
            .read_line(&mut palpite)
            .expect("Falha ao ler entrada");

        // match é um estrutura de controle
        // Ok() => O número é convertido
        // Err(_) => O número não é convertido pois é inválido
        // Cada => é chamado de braço

        let palpite: i32 = match palpite.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Digite um número válido!");
                continue;
            }
        };

        println!("Você disse: {}", palpite);

        // Compara o palpite com o número gerado
        // Retorna um enum 'Ordering'
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
