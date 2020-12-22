//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa que leia dois vetores A e B com 5 elementos. O vetor A deve aceitar apenas a entrada de valores divisíveis por 2 ou 3, enquanto o vetor B deve aceitar apenas a entrada de valores que não sejam múltiplos de 5. A entrada dos vetores deve ser validada pelo programa, e não pelo usuário. Construir um vetor C que seja o resultado da junção dos vetores A e B, de forma que contenha 10 elementos. Apresentar os elementos do vetor C.
use std::io;

fn main() {
    let mut numero_a = [0;5];
    let mut numero_b = [0;5];
    let mut numero_c = [0;10];

    let mut contador = 0;

    while contador < numero_a.len() {

        println!("Informe 5 números divisivel por 2 ou 3 ");

        let mut string_numero: String = String::new();
        io::stdin()
            .read_line(&mut string_numero)
            .expect("Falha ao ler o valor");

        let numero: i32 = converter_string_para_i32(string_numero);

        if numero % 2 == 0 || numero % 3 == 0 {
            numero_a[contador] = numero;
        } else{
            println!("Este numero nao é divisivel por 2 ou 3")
        }
        contador += 1;
    }

    contador = 0 ;

    while contador < numero_b.len() {

        println!("Informe 5 números não divisiveis por 5  ");

        let mut string_numero: String = String::new();
        io::stdin()
            .read_line(&mut string_numero)
            .expect("Falha ao ler o valor");

        let numero: i32 = converter_string_para_i32(string_numero);

        if numero  % 5 != 0 {
            numero_b[contador] = numero;
        };
        contador += 1 ;
    };


    contador = 0;
    // Juntando os vetores A e B no vetor C:
    while contador < numero_a.len() {
        numero_c[contador] = numero_a[contador];
        contador += 1;
    };
    

    contador = 0;
    while contador < numero_b.len() {
        numero_c[contador + 5] = numero_b[contador];
        contador += 1;
    };
  
    println!("Vetor A: {:?}", numero_a);
    println!("Vetor B: {:?}", numero_b);
    println!("Vetor A e B: {:?} {:?}", numero_a, numero_b);
    println!("Vetor C: {:?}", numero_c);
   

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}
