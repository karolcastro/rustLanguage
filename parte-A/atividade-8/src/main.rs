///A sequência de Fibonacci é formada pelos números:
//0, 1, 2, 3, 5, 8, 13, 21, … 
///Sendo o primeiro número da série 0, o segundo 1, e os subsequentes 
/// a soma dos dois anteriores. Escrever um programa que imprima os 13 primeiros termos da série.
/// 
/// nao deu

fn main() {
    println!("O valor do 13o. elemento da sequência Fibonnaci é: {}", fibonacci(13));
}

fn fibonacci(contador : i32) -> i32 {
    if contador <= 1 {
            return contador;
    } else {
        fibonacci(contador - 1) + fibonacci(contador - 2)
    }
}