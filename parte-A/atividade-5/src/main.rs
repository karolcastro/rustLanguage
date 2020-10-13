///Construir um programa que leia um valor numérico inteiro e apresente como resultado armazenado
/// os seus valores sucessor e antecessor.
///
use std::io;

fn main() {
    println!("Digite um número");

    let mut inteiro = String::new();
    io::stdin()
        .read_line(&mut inteiro)
        .expect("Failed to read line");

    let inteiro1: i32 = converter_string_para_i32(inteiro);

    let antecessor = inteiro1 - 1;
    let sucessor = inteiro1 + 1;

    if inteiro1 > 0 {
        println!("O antecessor a este número é: {}", antecessor);
        println!("O sucessor a este número é: {}", sucessor)
    } else {
        print!("Digite um número maior que 0")
    }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}
