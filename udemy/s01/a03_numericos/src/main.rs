fn main() {
    let chassi = 12345;
    let acel_max = 3.0;
    let acel_min = -10.0;
    let comprimento = 4;
    let posicao_atual = -100.0;
    let velocidade_atual = 0.0;
    let acel_atual = 0.0;

    // let teste = chassi + acel_max;
    //
    let teste = chassi + acel_max as i32;

    let xxx: f64 = 123.33;

    println!(
        "trunc {}, round {}, ceil {}, floor {}",
        xxx.trunc(),
        xxx.round(),
        xxx.ceil(),
        xxx.floor()
    );

    let tup1 = (23, 4.0, "lala", false);
    let tup2: (i32, f64, i32) = (42, 42.2, 22);
    let (a1, a2, b2) = tup2;

    println!("Minha tupla tem: {:?} {:?} {:?}", tup2.0, tup2.1, tup2.2);

    let lista1 = [1, 4, 5, 99, 9];
    let lista2 = [23; 4]; // array com 23 23 23 23
}
