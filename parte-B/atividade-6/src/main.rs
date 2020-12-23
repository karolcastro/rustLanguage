//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa para calcular o máximo divisor comum de dois números inteiros positivos, MDC(x, y), usando o algoritmo de Euclides
// Este algoritmo é baseado no fato de que se o resto da divisão de x por y representado por r, for igual a zero, y é o MDC. 
//Se o resto r for diferente de zero, o MDC de x e y é igual ao MDC de y e r. 
//O processo se repete até que o valor do resto da divisão seja zero.

// nao consegui
use std::io;
fn main() {

        println!("Informe o primeiro numero:");
    
        let mut string_numero1: String = String::new();
        io::stdin()
            .read_line(&mut string_numero1)
            .expect("Falha ao ler o valor");
        let mut primeiro: i32 = converter_string_para_i32(string_numero1);

        println!("Informe o segundo numero:");
    
        let mut string_numero2: String = String::new();
        io::stdin()
            .read_line(&mut string_numero2)
            .expect("Falha ao ler o valor");
        let mut segundo: i32 = converter_string_para_i32(string_numero2);
    
        let mut contador :i32 = 0;
        
        // println!("Os números inforamdos são: {} {}", numero_primeiro, numero_segundo);

        
        let mut resto:i32;

        while segundo != 0 {
            resto = primeiro % segundo;
            primeiro = segundo;
            segundo = resto;
        }
        println!("O máximo divisor comum é {}", primeiro);
    }

        fn converter_string_para_i32(uma_string: String) -> i32 {
            uma_string
                .trim()
                .parse()
                .expect("Falha ao converter o valor")
        }
    
