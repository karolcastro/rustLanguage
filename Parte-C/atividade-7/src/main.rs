/*Elaborar um programa que leia uma matriz A e B do tipo inteiro de cinco colunas e cinco linhas. A matriz A deve ser formada por valores divisíveis por 3 e 4, enquanto a matriz B deve ser formada por valores divisíveis por 5 e 6. As entradas do usuário deverão ser validadas pelo programa. Construir e apresentar uma matriz C de mesma dimensão que contenha o valor da multiplicação dos elementos da matriz A com os elementos correspondentes da matriz B. Apresentar as três matrizes. */


use std::io;
fn main() {

let coluna_a = 5;
let linha_a = 5;
let coluna_b = 5;
let linha_b = 5;
let coluna_c = 5;
let linha_c = 5;

let mut a = vec![vec![0; coluna_a]; linha_a];
let mut b = vec![vec![0; coluna_b]; linha_b];
let mut c = vec![vec![0; coluna_b]; linha_b];
    
for i in 0..linha_a {
    for j in 0..coluna_a {
        println!("Digite um número para A");
        let mut numero =  String::new();

        io::stdin()
       .read_line(&mut numero)
       .expect("Failed to read line");

       let mut numero1: i32 = converter_string_para_i32(numero);

        if a[i][j] % 3 == 0 && a[i][j] % 4 == 0 {
            a[i][j] = numero1;
        }else{
            a[i][j]= 0;
        }
        
    }
}
println!("Matriz A: {:?}", a);

for i in 0..linha_b {
   for j in 0..coluna_b {
    println!("Digite um número para B");
    let mut numero =  String::new();

    io::stdin()
   .read_line(&mut numero)
   .expect("Failed to read line");

   let mut numero2: i32 = converter_string_para_i32(numero);
    
//    b[i][j] = numero2;
    if b[i][j] % 5 == 0 && b[i][j] % 6 == 0 {
        b[i][j] = numero2;
       }else{
        b[i][j] = 0;
       }
   }
}
println!("Matriz B: {:?}", b);

for i in 0..linha_c {
    for j in 0..coluna_c {
        c[i][j] = a[i][j] * b[i][j];
    }
}
println!("Matriz A: {:?}", a);
println!("Matriz B: {:?}", b);
println!("Matriz C: {:?}", c);

fn converter_string_para_i32(uma_string: String) -> i32 {
   uma_string
       .trim()
       .parse()
       .expect("Falha ao converter o valor")
}
}
