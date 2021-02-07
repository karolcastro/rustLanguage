/*Elaborar um programa que leia duas matrizes A e B, cada uma de duas dimensões com três linhas e três colunas para valores inteiros. Construir uma matriz C de mesma dimensão, que seja formada da soma dos elementos da matriz A com os elementos da matriz B. Apresentar os elementos da matriz C.*/


fn main() {

let a = vec![ 2, 4, 6];
let b = vec![ 8, 10, 12];
let mut c = vec![a; b];

for x in 0..a{
    for z in 0..b{
        println!("{:?}", c[x][z]) 
    }
}
}


