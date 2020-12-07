//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa que leia dois vetores A e B com 5 elementos. O vetor A deve aceitar apenas a entrada de valores divisíveis por 2 ou 3, enquanto o vetor B deve aceitar apenas a entrada de valores que não sejam múltiplos de 5. A entrada dos vetores deve ser validada pelo programa, e não pelo usuário. Construir um vetor C que seja o resultado da junção dos vetores A e B, de forma que contenha 10 elementos. Apresentar os elementos do vetor C.

fn main() {
    

    let mut numero_a = 0;
    let mut numero_b = 0;
    // let numero_c = numero_a[0..5] + numero_b[0..5];

    
    while numero_a <= 5 {
        if numero_a % 2 == 0 || numero_a % 3 == 0{
             println!("Números divisíveis por 2 e 3: {}", numero_a);
        };
        numero_a += 1;
            };

            while numero_b <=5 {
                if numero_b == !0 || numero_b == !5{
                    println!("Números não divisíveis por 5: {:?}", numero_b)
                };
                numero_b +=1;
            }
            // println!("{}", numero_c)
        }

