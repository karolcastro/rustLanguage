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

            a[i][j] = numero1;
            if numero1 % a[i][j] == 1 { //TODO
                println!("Matriz A: {:?}", numero1);
            }
        }
        println!("Matriz A: {:?}", numero1);
        // println!("Matriz A: {:?}", a);
    }
    
    fn converter_string_para_i32(uma_string: String) -> i32 {
       uma_string
           .trim()
           .parse()
           .expect("Falha ao converter o valor")
    }
    }


