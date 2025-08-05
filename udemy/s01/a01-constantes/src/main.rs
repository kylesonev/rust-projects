const UMA_HORA_EM_SEGUNDOS: i32 = 1 * 60 * 60;
fn main() {
    println!("Início do programa");

    let mut x = 5;
    println!("O valor de x é {x}");

    x = UMA_HORA_EM_SEGUNDOS;

    println!("Agora o x é: {x}");
}
