/*Elaborar um programa que leia dois vetores A e B, para 5 elementos reais. Construir uma matriz C de duas dimensões, sendo a primeira coluna da matriz C formada pelos elementos do vetor A multiplicados por 2 e a segunda coluna formada pelos elementos do vetor B subtraídos por 5. Apresentar a matriz C.*/

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
       c[j][0] = a[i][j]*2;
       c[j][1] = b[i][j]-5;
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
