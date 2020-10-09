///A sequência de Fibonacci é formada pelos números:
//0, 1, 2, 3, 5, 8, 13, 21, … 
///Sendo o primeiro número da série 0, o segundo 1, e os subsequentes 
/// a soma dos dois anteriores. Escrever um programa que imprima os 13 primeiros termos da série.
/// 
/// nao deu

use std::io;


fn main() {

    println!("Escolha um número de 0 a 100 ");

    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n1: i32 = converter_string_para_i32(n);

    let fibonacci = n1 + 1;

    if n1 == 0 {
        println!(" Sequencia de Fibonacci {}", n1);
    }
    if n1 >= 0  {
        println!(" Sequencia de Fibonacci {}", fibonacci);
    }
    else {
        println!(" Nao entendi! ");
    }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}
