fn main() {
    println!("Início do programa!");

    const X: i32 = 5;
    let y = 6;
    let mut z = 7;
    z = z + 1;

    println!("No início os valores são: X={X}, y={y} e z={z}");

    {
        const X: i32 = 67;
        let y = 24;
        z = z + 23;
        println!("Dentro do bloco são: X={X}, y={y} e z={z}");
    }

    println!("Depois: X={X}, y={y} e z={z}");
}
