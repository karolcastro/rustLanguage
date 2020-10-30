///Efetuar a leitura de quatro valores numéricos inteiros, 
/// representados pelas variáveis A, B, C e D. 
/// Apresentar apenas os valores que sejam divisíveis por e 2 e 3.

fn main() {
    

    let a1 = 18;
    let b1 = 30;
    let c1 = 6;
    let d1 = 9;


    if a1 % 2 == 0 && a1 % 3 == 0 {
        println!("Número divisivel por 2 e 3: {}", a1);
    }

    if b1 % 2 == 0 && b1 % 3 == 0  {
        println!("Número divisivel por 2 e 3: {} ",b1);
 
    }
    if c1 % 2 == 0 && c1 % 3 == 0 {
        println!("Número divisivel por 2 e 3: {} ", c1);
    }
    if d1 % 2 == 0 && d1 % 3 == 0 {
        println!("Número divisivel por 2 e 3: {}", d1);

    }

}
