///Efetuar a leitura de quatro valores numéricos inteiros, 
/// representados pelas variáveis A, B, C e D. 
/// Apresentar apenas os valores que sejam divisíveis por e 2 e 3.

use std::io;

fn main() {
    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Failed to read line");

    let a1: f32 = converter_string_para_f32(a);


    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Failed to read line");

    let b1: f32 = converter_string_para_f32(b);

    let mut c = String::new();
    io::stdin()
        .read_line(&mut c)
        .expect("Failed to read line");

    let c1: f32 = converter_string_para_f32(c);

    let mut d = String::new();
    io::stdin()
        .read_line(&mut d)
        .expect("Failed to read line");

    let d1: f32 = converter_string_para_f32(d);

    let a1 = 2;
    let b1 = 3;
    let c1 = 4;
    let d1 = 9;


    if a1 % 2 == 0  {
        println!("Número divisivel por 2: {} {} {} {}", a1, b1, c1, d1);
    }

    if b1 % 2 == 0  {
        println!("Número divisivel por 2: {} {} {} {}", a1, b1, c1, d1);
    }
    if c1 % 2 == 0  {
        println!("Número divisivel por 2: {} {} {} {}", a1, b1, c1, d1);
    }
    if d1 % 2 == 0  {
        println!("Número divisivel por 2: {} {} {} {}", a1, b1, c1, d1);
    }
    else {
        println!(" Não entendi ! ");
    }

    fn converter_string_para_f32(uma_string: String) -> f32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}
