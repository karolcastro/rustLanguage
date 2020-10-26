///A sequência de Fibonacci é formada pelos números:
//0, 1, 2, 3, 5, 8, 13, 21, … 
///Sendo o primeiro número da série 0, o segundo 1, e os subsequentes 
/// a soma dos dois anteriores. Escrever um programa que imprima os 13 primeiros termos da série.
/// 
/// nao deu

    use std::io;

    fn main() {
    
        let mut fibonacci = String::new();
        io::stdin()
            .read_line(&mut fibonacci)
            .expect("Failed to read line");
        
        let fib : f32 = converter_string_para_f32(fibonacci);
        
        let n = 3
    
        let fibo = fib (n-1) + fib (n-2);
    
        if n <= 0 { 
            println!(" 0 ");
        
        }else if n == 1 {
            println!(" 1 ");
        }
        else {
            println!(" 1 {}",fib )
        }
    
        fn converter_string_para_f32(uma_string: String) -> f32 {
            uma_string
                .trim()
                .parse()
                .expect("Falha ao converter o valor")
        }
    }