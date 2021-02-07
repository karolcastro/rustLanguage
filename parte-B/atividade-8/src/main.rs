//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa que leia um vetor A de 5 elementos inteiros. Construir um vetor B do mesmo tipo, e 
//cada elemento do vetor B deve ser o resultado da fatorial correspondente de cada elemento do vetor A. Apresentar os vetores A e B.

fn main() {
    let a = [1, 2, 3, 4, 5];
    let b = [(a[0] * a[0]),
    (a[0] * a[1]),
    (a[2] * a[1] * a[0]),
    (a[3] * a[2] * a[1] * a[0]),
    (a[4] * a[3] * a[2] * a[1] * a[0])];
    let index = 0;

    while index < 6 {
        println!("Os elementos de A são {}, {}, {}, {} e {}", a[0],a[1],a[2],a[3],a[4]);
        println!("Os fatoriais de A são: de {} é {}, de {} é {}, de {} é {}, de {} é {} e de {} é {}.",a[0], b[0],a[1],b[1],a[2],b[2], a[3],b[3],a[4],b[4]);
       break;
    };
}
