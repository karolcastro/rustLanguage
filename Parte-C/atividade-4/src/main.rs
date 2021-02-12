/*Elaborar um programa que leia uma matriz A do tipo real de duas dimensões com cinco linhas e cinco colunas. Apresentar o somatório dos elementos situados na diagonal principal.*/


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

           if i == j {
            a[i][j] = numero1;
            a[i][j] =  numero1.iter().sum();
        }
        
        }
    }
    println!("Matriz A: {:?}", a);
    
    fn converter_string_para_i32(uma_string: String) -> i32 {
       uma_string
           .trim()
           .parse()
           .expect("Falha ao converter o valor")
    }
    }
