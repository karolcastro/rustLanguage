// nao faz a potencia pq nao consigo incrementar +=1

// Elaborar um programa que leia dois valores numéricos inteiros, os quais devem representar 
//a base e o expoente de uma potência. 
//Calcular a potência, e exibir a operação calculada, e o resultado obtido.

use std::io;

fn main() {
    println!("Digite o valor de base" );

    let mut base = String::new();

    io::stdin()
        .read_line(&mut base)
        .expect("Failed to read line");

    let mut base1: i32 = converter_string_para_i32(base);

    println!("Digite o valor de potencia ");

    let mut potencia = String::new();

    io::stdin()
        .read_line(&mut potencia)
        .expect("Failed to read line");

    let  mut potencia1: i32 = converter_string_para_i32(potencia);

    
    if potencia1 == 0 {
        println!( " A potencia é: 1" );   
            
    }else if potencia1 > 0 {
        println!("A pontencia é: {}", base1 * potencia1 [++1])
    
    }else if potencia1 == 1 {
        println!("A pontencia é: {:?}", base1 )
        
    } else{
        print!("Números não adequados");
    }



    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}