//Elaborar um programa que mostre os resultados da tabuada, de 0 até 10, de um número qualquer.
//Restrição: Utilize o laço condicional pré-teste.


use std::io;

fn main() {

    println!("Informe a tabuada que deseja ver:");

    let mut string_numero: String = String::new();
    io::stdin()
        .read_line(&mut string_numero)
        .expect("Falha ao ler o valor");
    let numero: i32 = converter_string_para_i32(string_numero);

    let mut contador :i32 = 0;
    
    println!("A tabuada do {} é:", numero);

    while contador <= 10 {
        println!("{}x{}={}", numero,contador, numero * contador);
        contador += 1;
    }   
    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}

