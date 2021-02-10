/*Elaborar um programa que leia uma matriz A 5x5. Construir uma matriz B 5x5 em que cada elemento seja o dobro correspondente à matriz A, com exceção dos elementos da diagonal principal, os quais devem ser o triplo de cada elemento correspondente da matriz A. Apresentar a matriz B.*/


use std::io;
fn main() {

let coluna_a = 5;
let linha_a = 5;
let coluna_b = 5;
let linha_b = 5;

let mut a = vec![vec![0; coluna_a]; linha_a];
let mut b = vec![vec![0; coluna_b]; linha_b];
    
for i in 0..linha_a {
    for j in 0..coluna_a {
        println!("Digite um número para A");
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
       if i == j {
           b[i][j] = a[i][j] * 3;
       }
       else {
        b[i][j] = a[i][j] * 2;
       }
   }
   
}
println!("Matriz B: {:?}", b);


fn converter_string_para_i32(uma_string: String) -> i32 {
   uma_string
       .trim()
       .parse()
       .expect("Falha ao converter o valor")
}
}
