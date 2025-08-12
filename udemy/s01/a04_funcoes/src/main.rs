fn outra_funcao() {
    println!("Essa não recebe parâmetro...");
}

fn outra_funcao_parametro(x: i32) {
    println!("Número recebido: {x}");
}

fn dois_parametros(valor: f64, unidade: char) {
    println!("A medida é: {valor}{unidade}");
}

fn soma(x: i32, y: i32) -> i32 {
    x + y
}

fn soma_ret(x: i32, y: i32) -> i32 {
    return x + y;
}

fn main() {
    outra_funcao();
    outra_funcao_parametro(33);
    dois_parametros(239.2, 'C');

    let xy = soma(32, 11);

    println!("Sem return: {xy}");
    println!("Com return: {}", soma_ret(100, 10));
}
