////A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.
/// 
/// Elaborar um programa que leia 10 elementos (valores reais) para temperatura em graus Celsius e armazene esses valores em um vetor A. O programa ao final deve apresentar a menor, a maior e a média das temperaturas lidas.

fn main() {
    let temp = [34.5, 32.5, 28.3, 25.4, 30.0, 31.0, 33.3, 26.6, 27.6, 27.8];
    let mut contador = 0;
    let media = (temp[0] + temp[1] + temp[2] + temp[3] + temp[4] + temp[5] + temp[6] + temp[7] + temp[8] + temp[9]) / 10.0;
   

    while contador < 10 {
        println!(" A média das temperaturas é: { } ",media);
        println!(" A maior temperaturas é: { } ",temp.len());
        println!(" A menor temperatura é: { } ",media);
        break;
    }
    

}
