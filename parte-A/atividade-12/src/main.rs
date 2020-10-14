//Efetuar a leitura de três valores inteiros desconhecidos representados pelas variáveis A, B e C.
//Somar os valores fornecidos e apresentar o resultado somente se for maior ou igual a 100.

use std::io;

fn main() {
    println!("Informe o número A: ");

    let mut a: String = String::new();

    io::stdin()
    .read_line(&mut a)
    .expect("Falha ao ler o valor");

    let numeroA: i32 = converter_string_para_i32(a);

    println!("Informe o número B: ");

    let mut b = String::new();

    io::stdin()
    .read_line(&mut b)
    .expect("Falha ao ler o valor");

    let numeroB: i32 = converter_string_para_i32(b);


    println!("Informe o número C: ");

    let mut c = String::new();

    io::stdin()
    .read_line(&mut c)
    .expect("Falha ao ler o valor");

    let numeroC: i32 = converter_string_para_i32(c);

    println!("Informe o número D: ");

    let mut d = String::new();

    io::stdin()
    .read_line(&mut d)
    .expect("Falha ao ler o valor");

    let numeroD: i32 = converter_string_para_i32(d);

    let soma = ((numeroA + numeroB) + numeroC) + numeroD;

    if soma <= 100 {
        println!("A soma dos números foi: {}", soma);
    } else {
        println!("A soma dos números foi maior que 100 ");
    }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}
