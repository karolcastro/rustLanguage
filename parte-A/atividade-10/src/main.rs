//Ler um valor numérico inteiro e apresente uma 
//mensagem informando se o valor fornecido é par ou ímpar
use std::io;

fn main() {

    println!("Digite um número ");

    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    let numero: u32 = converter_string_para_u32(num1);



    if numero % 2 == 0 {
        println!("O número que você digitou é par {}",numero );
    }

   else {
        println!("O número que você digitou é ímpar {}",numero );
    }


    fn converter_string_para_u32(uma_string: String) -> u32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
    
}
