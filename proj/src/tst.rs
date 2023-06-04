use std::io;

fn main() {
    let mut s = String::new();
    println!("Digite um numerozinhooo");

    io::stdin()
        .read_line(&mut s)
        .expect("Error na digitacao broo");

    println!("Voce colocou {s}")
}