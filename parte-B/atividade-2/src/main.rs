//Elaborar um programa que apresente todos os valores numéricos inteiros ímpares situados na faixa de 0 a 20.
//Restrição: Utilize o laço incondicional.

fn main() {
   
    for numero in 0..21 {
        if numero % 2 == 1 {
            print!(" {} ", numero);
        }
    }
}

