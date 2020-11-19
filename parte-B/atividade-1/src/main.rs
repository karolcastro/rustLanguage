//Elaborar um programa que mostre os resultados da tabuada, de 0 até 10, de um número qualquer.
//Restrição: Utilize o laço condicional pré-teste.

fn main() {
    let mut numero = 3;

    while numero <= 30 {
        if numero % 3 == 0 {
            print!("{} ", numero)
        }
        numero += 1;
    }
}
