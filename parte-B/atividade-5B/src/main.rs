//Elaborar um programa que leia 3 valores inteiros, armazene-os em um vetor, e percorra o vetor calculando o fatorial de cada valor lido. No final, deverá apresentar o somatórios dos valores calculados.

//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

fn main() {

    let mut contador = 1;
    let mut resultado_fatorial = 1;

    while contador <= 2
     {

        resultado_fatorial = resultado_fatorial * contador;
        contador = contador + 1;
    }
    // esta retornando outro valor ... 120
    println!("O resultado é {:?}", resultado_fatorial)

}
