//Escrever um programa que receba três números inteiros como entrada 
//e imprima, como saída, os números em ordem crescente.

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

    
    if numero1 <= numero2 && numero1 <= numero3 && numero2 <= numero3{
        println!("A sequencia é: {} {} {}", numero1, numero2, numero3);
    }
    if numero1 <= numero2 && numero1 <= numero3 && numero3 <= numero2 {
        println!("A sequencia é: {} {} {}", numero1, numero3, numero2);
    }
    if numero2 <= numero1 && numero2 <= numero3 && numero1 <= numero3 {
        println!("A sequencia é: {} {} {}", numero2, numero1, numero3);
    }
    if numero2 <= numero1 && numero2 <= numero3 && numero3 <= numero1 {
        println!("A sequencia é: {} {} {}", numero2, numero3, numero1);
    }
    if numero3 <= numero1 && numero3 <= numero2 && numero1 <= numero2 {
        println!("A sequencia é: {} {} {}", numero3, numero1, numero2);
    }
    if numero3 <= numero1 && numero3 <= numero2 && numero2 <= numero1 {
        println!("A sequencia é: {} {} {}", numero3, numero2, numero1);
    }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}
