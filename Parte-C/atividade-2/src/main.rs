/*Elaborar um programa que leia dois vetores A e B, para 5 elementos reais. Construir uma matriz C de duas dimensões, sendo a primeira coluna da matriz C formada pelos elementos do vetor A multiplicados por 2 e a segunda coluna formada pelos elementos do vetor B subtraídos por 5. Apresentar a matriz C.*/

use std::io;
fn main() {

// let coluna_a = 2;
// let linha_a = 5;
// let coluna_b = 2;
// let linha_b = 5;
let coluna_c = 2;
let linha_c = 5;

let mut contador = 0;
let mut a = [0;5];
let mut b = [0;5];
let mut c = vec![vec![0; coluna_c]; linha_c];
    
// for i in 0..linha_a {
//     for j in 0..coluna_a {

    while contador < 5 {

        println!("Digite um número para A");
        let mut numero =  String::new();

        io::stdin()
       .read_line(&mut numero)
       .expect("Failed to read line");

       let  numero1: i32 = converter_string_para_i32(numero);
        a[contador] = numero1;
        contador += 1;
    }

println!("Vetor A: {:?}", a);

contador = 0;
// for i in 0..linha_b {
//    for j in 0..coluna_b {

    while contador < 5 {

       println!("Digite um número para B");
       let mut numero =  String::new();

       io::stdin()
      .read_line(&mut numero)
      .expect("Failed to read line");

      let  numero2: i32 = converter_string_para_i32(numero);
       b[contador] = numero2;
       contador += 1;
   }
   
println!("Vetor B: {:?}", b);

for i in 0..linha_c {
        
        c[i][0] = a[i] * 2;
        c[i][1] = b[i] - 5;
}
    

println!("A matriz C é : {:?}", c);

fn converter_string_para_i32(uma_string: String) -> i32 {
   uma_string
       .trim()
       .parse()
       .expect("Falha ao converter o valor")
}
}
