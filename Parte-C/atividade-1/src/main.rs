/*Elaborar um programa que leia duas matrizes A e B, cada uma de duas dimensões com três linhas e três colunas para valores inteiros. Construir uma matriz C de mesma dimensão, que seja formada da soma dos elementos da matriz A com os elementos da matriz B. Apresentar os elementos da matriz C.*/

use std::io;
fn main() {

let coluna_a = 3;
let linha_a = 3;
let coluna_b = 3;
let linha_b = 3;
let coluna_c = 3;
let linha_c = 3;

let mut a = vec![vec![0; coluna_a]; linha_a];
let mut b = vec![vec![0; coluna_b]; linha_b];
let mut c = vec![vec![0; coluna_c]; linha_c];

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
println!("Matriz A: {:?}", a);

for i in 0..linha_b {
    for j in 0..coluna_b {
        println!("Digite um número");
        let mut numero =  String::new();

        io::stdin()
       .read_line(&mut numero)
       .expect("Failed to read line");

       let  numero2: i32 = converter_string_para_i32(numero);
        b[i][j] = numero2;
    }
    
}
println!("Matriz B: {:?}", b);

for i in 0..linha_c {
    for j in 0..coluna_c {
        c[i][j] = a[i][j] + b[i][j];
    }
}
println!("A matriz C é : {:?}", c);

fn converter_string_para_i32(uma_string: String) -> i32 {
    uma_string
        .trim()
        .parse()
        .expect("Falha ao converter o valor")
}
}


