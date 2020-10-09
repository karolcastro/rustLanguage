use std::io;

fn main() {
    println!("Digite a cotação do dolár (U$) de hoje em reais (R$)" );

    let mut cotacao = String::new();

    io::stdin()
        .read_line(&mut cotacao)
        .expect("Failed to read line");

    let cotacao1: f32 = converter_string_para_f32(cotacao);

    println!("Digite o valor em reais (R$) para conversão ");

    let mut valor = String::new();

    io::stdin()
        .read_line(&mut valor)
        .expect("Failed to read line");

    let  valor1: f32 = converter_string_para_f32(valor);

    
    if valor1 > cotacao1 {
        println!( "O total em reais é: R$ {}", cotacao1 * valor1 )        
    }else{
        print!("Cotação do dolar (U$) maior que o valor para conversão")
    }

    fn converter_string_para_f32(uma_string: String) -> f32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}