//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa que leia dois vetores (denominados A e B) com 5 elementos reais. Construir um vetor denominado C, onde cada elemento deverá corresponder a subtração do elemento correspondente do vetor A com um elemento correspondente do vetor B. Ao final, apresentar os elementos do vetor C.

fn main() {
    let a = [2.0, 4.0 ,6.0, 8.0, 10.0];
    let b = [1.0, 3.0, 5.0, 7.0, 9.0];
    let index = 0;
    let c = [
        a[0] - b[0], 
        a[1] - b[1],
        a[2] - b[2],
        a[3] - b[3],
        a[4] - b[4]
    ];
    while index < 5 {
        println!("{} {} {} {} {}", c[0],c[1],c[2],c[3],c[4]);
        break;
    }
}
