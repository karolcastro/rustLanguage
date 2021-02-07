use std::io;

fn main() {
    println!("Digite o valor de A");

    let mut a = String::new();
    io::stdin()
    .read_line(&mut a)
    .expect("Failed to read line");

    let a1: f32 = converter_string_para_f32(a);

    println!("Digite o valor de B ");

    let mut b = String::new();
    io::stdin()
    .read_line(&mut b)
    .expect("Failed to read line");

    let b1: f32 = converter_string_para_f32(b);

    let divisao = a1 / b1;
    let multiplicacao = a1 * b1;
    let adicao = a1 + b1;
    let subtracao = a1 - b1;

    if a1 >= b1 || a1 == b1 || a1 <= b1 {np
        println!(" Divisão de A e B é: {}", divisao);
        println!(" Multiplicao de A e B é: {}", multiplicacao);
        println!(" Adição de A e B é: {}", adicao);
        println!(" Subtração de A e B é: {}", subtracao)
    }
    
    else {
        print!("Digite um número válido !")
    }

    fn converter_string_para_f32(uma_string: String) -> f32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}
