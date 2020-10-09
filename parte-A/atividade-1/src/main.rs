///Elaborar um programa que calcule e apresente o valor do volume 
/// de uma caixa retangular, utilizando a fórmula:
///Volume ← Comprimento * Largura * Altura


use std::io;

fn main() {
    println!("Digite a Altura");

    let mut altura = String::new();

    io::stdin()
        .read_line(&mut altura)
        .expect("Failed to read line");

    let altura1: i32 = converter_string_para_i32(altura);

    println!("Digite a Largura");

    let mut largura = String::new();

    io::stdin()
        .read_line(&mut largura)
        .expect("Failed to read line");

    let  largura1: i32 = converter_string_para_i32(largura);


    println!("Digite o comprimento");

    let mut comprimento = String::new();

    io::stdin()
        .read_line(&mut comprimento)
        .expect("Failed to read line");

    let  comprimento1: i32 = converter_string_para_i32(comprimento);

    let volume = (comprimento1 * largura1) * altura1;
    
        println!( "O volume da caixa é: {}", volume );        


    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}