use std::io;

fn main() {
    println!("Digite o início da contagem regressiva: ");

    let mut entrada = String::new();
    io::stdin()
        .read_line(&mut entrada)
        .expect("Erro ao ler entrada");

    let mut repet: i32 = entrada.trim().parse().expect("Número inválido!");

    let mut numeros: Vec<i32> = Vec::new();

    while repet != 0 {
        numeros.push(repet);
        println!("Repetindo... {repet}");
        repet -= 1;
    }

    for num in numeros {
        println!("Número: {num}");
    }

    for number in 1..4 {
        println!("{number}... ");
    }

    for numerin in 1..=4 {
        print!("{numerin}... ");
    }

    for lala in (1..=4).rev() {
        print!("{lala}...");
    }
}
