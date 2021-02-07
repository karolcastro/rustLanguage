//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa que leia 10 elementos inteiros e armazene em um vetor A. Apresentar os valores do vetor lido de maneira inversa, e identificar qual é o menor valor armazenado.

fn main() {
    let a = &[0;10];
    let mut menor_numero = a[0];

    
    for a in (1..11).rev(){
        println!("{} ", a)
    }
    for i in a.iter(){
        if i < &menor_numero{
            menor_numero = *i;
        }
    }
   
}

