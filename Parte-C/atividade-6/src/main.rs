/*Elaborar um programa que leia uma matriz A do tipo inteiro de cinco colunas e cinco linhas. Ao final, apresentar o total de elementos pares e ímpares existentes na matriz. Apresentar também o percentual de elementos pares e ímpares em relação ao total de elementos.*/


use std::io;
fn main() {
    
    let coluna_a = 5;
    let linha_a = 5;
    
    let mut a = vec![vec![0; coluna_a]; linha_a];
        
    for i in 0..linha_a {
        for j in 0..coluna_a {
            println!("Digite um número");
            let mut numero =  String::new();
    
            io::stdin()
           .read_line(&mut numero)
           .expect("Failed to read line");
    
           let  numero1: i32 = converter_string_para_i32(numero);

            if i % 2 == 0 && j % 2 == 0 { 
                a[i][j] = numero1;
                // println!("Elementos pares são: {:?}", a.len());
            }
            else {
                // println!("Elementos ímpares são: {:?}", a);
            }
        }
    }
    println!("Elementos pares são: {:?}", a.len());
    println!("Elementos ímpares são: {:?}", a);
    
    fn converter_string_para_i32(uma_string: String) -> i32 {
       uma_string
           .trim()
           .parse()
           .expect("Falha ao converter o valor")
    }
    }


