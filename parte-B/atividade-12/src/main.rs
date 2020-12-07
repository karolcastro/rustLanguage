//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa que leia um vetor A com dez elementos numéricos inteiros. Apresentar o total de elementos ímpares existente no vetor e também o percentual do valor total de números ímpares em relação à quantidade total de elementos armazenados.

fn main() {
let mut numero: i32 = 0; 
let mut total_impar = 0;
    
  while numero <= 10{
      if numero %2 == 1{
          total_impar = total_impar+1;

      }
      numero +=1;
  }
  println!("O total de ímpares é igual a: {}", total_impar);
  println!("O percentual total é: {}", total_impar/100)
}
