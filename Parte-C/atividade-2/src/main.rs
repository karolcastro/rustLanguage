/*Elaborar um programa que leia dois vetores A e B, para 5 elementos reais. Construir uma matriz C de duas dimensões, sendo a primeira coluna da matriz C formada pelos elementos do vetor A multiplicados por 2 e a segunda coluna formada pelos elementos do vetor B subtraídos por 5. Apresentar a matriz C.*/


fn main() {
    let mut a = vec![ 2, 4, 6, 8, 10];
    let mut b = vec![ 8, 10, 12, 14, 16];
    

    for i in a.iter() {
          i * 2;
    }
        for i in b.iter(){
        i -5;
        // let c = primeira_coluna + segunda_coluna;
println!("{:?} {:?}", a, b)
// println!("{:?}", c)
        }
    }
