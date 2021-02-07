//Elaborar um programa que leia 3 valores inteiros, armazene-os em um vetor, e percorra o vetor calculando o fatorial de cada valor lido. No final, deverá apresentar o somatórios dos valores calculados.

//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

fn main() {
    let  v = [1,2,3];
    let index = 0;
    let fatorial1 = v[0] * v[0];
    let fatorial2 = v[1] * v[0];
    let fatorial3 = v[2] * v[1] * v[0];
    let soma = fatorial1 + fatorial2 + fatorial3;
    

    while index < 3{
    
        println!(" O fatorial de {} é = {}", v[0],fatorial1 );
        println!(" O fatorial de {} é = {}", v[1], fatorial2 );
        println!(" O fatorial de {} é = {}", v[2], fatorial3 );
        println!(" A soma dos valores é {}", soma );
       break;
    }
}


