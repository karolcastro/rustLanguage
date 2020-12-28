//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa que leia um vetor A com dez elementos numéricos inteiros. Apresentar o total de elementos ímpares existente no vetor e também o percentual do valor total de números ímpares em relação à quantidade total de elementos armazenados.

use std::io;

fn main() {

  let  mut vetor_a = [0;10];
  let mut contador = 0;
  let mut sum:i32 = vetor_a.iter().sum();
  // let total_impar:i64 = [0; 10].iter().sum();
  // let  total_impar:i64 = vetor_a.sum();

  while contador < vetor_a.len() {

    println!("Informe 10 números: ");

  let mut string_numero: String = String::new();
  io::stdin()
    .read_line(&mut string_numero)
    .expect("Falha ao ler o valor");
  
  let numero: i32 = converter_string_para_i32(string_numero);
    
    if numero % 2 == 1 {
      vetor_a[contador] = numero;
    };
    contador += 1;
  }

  println!("O total de ímpares é igual a: {:?}", vetor_a);
  // println!("O total de ímpares é igual a: {}", vetor_a.len());
  // println!("O percentual total é: {}", vetor_a / 100);

  fn converter_string_para_i32(uma_string: String) -> i32 {
    uma_string
      .trim()
      .parse()
      .expect("Falha ao converter o valor")
  }
}



