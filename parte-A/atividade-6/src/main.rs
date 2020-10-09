//fn main() {
//  println!("Hello, world!");
//};

use std::io;

fn main() {
    println!("Digite o primeiro número");

    let mut primeiro = String::new();
    io::stdin()
        .read_line(&mut primeiro)
        .expect("Failed to read line");

    let numero1: i32 = converter_string_para_i32(primeiro);

    println!("Digite o segundo número");

    let mut segundo = String::new();
    io::stdin()
        .read_line(&mut segundo)
        .expect("Failed to read line");

    let numero2: i32 = converter_string_para_i32(segundo);

    println!("Digite o terceiro número");

    let mut terceiro = String::new();
    io::stdin()
        .read_line(&mut terceiro)
        .expect("Failed to read line");

    let numero3: i32 = converter_string_para_i32(terceiro);

    if numero1 <= numero2 && numero1 <= numero3 {
        println!("A sequencia é: {:?} {:?} {:?}", numero1, numero2, numero3);
    } else {
        print!("Digite uma sequencia válida")
    }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}