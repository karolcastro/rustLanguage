//Elaborar um programa que leia 3 valores inteiros, armazene-os em um vetor, e percorra o vetor calculando o fatorial de cada valor lido. No final, deverá apresentar o somatórios dos valores calculados.

//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

fn main() {
    let  v= [1,2,3];
    let mut index = 2;
    // let mut fatorial = index * v;
    

    while index < 3{
        println!(" O valor é {}", v[index] );
       index -=1;
    }
}
