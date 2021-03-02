/*Elaborar um programa que leia uma matriz A do tipo inteiro de cinco colunas e cinco linhas. Ao final, apresentar o total de elementos pares e ímpares existentes na matriz. Apresentar também o percentual de elementos pares e ímpares em relação ao total de elementos.*/


use std::io;
fn main() {
    
    let coluna_a = 5;
    let linha_a = 5;
    let mut soma = 0;
    
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

        }
    }

    let mut total_pares = 0.0;
    let mut total_impares = 0.0;
    let mut percent_pares = 0.0;
    let mut percent_impar = 0.0;
    let mut percent = 100.0;

    for i in 0..linha_a {
        for j in 0..coluna_a {
            // separa os elementos pares
            if a[i][j] % 2 == 0 {
                 total_pares += 1.0;
            }
            else {
                total_impares += 1.0;
            }
        }
    }

    for i in 0..linha_a {
        for j in 0..coluna_a {
            // separa os elementos pares
            if a[i][j] % 2 == 0 {
                 percent_pares = total_pares / 100.0;
            }
            else {
                percent_impar = total_impares / 100.0;
            }
        }
    }

    println!("Matriz A: {:?}", a);
    println!("Total Impar : {:?}", total_impares);
    println!("Total Par : {:?}", total_pares);
    println!("Percentual Impar : {:?}", percent_impar);
    println!("Percentual Par : {:?}", percent_pares);
    

    }
    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
     }
    

