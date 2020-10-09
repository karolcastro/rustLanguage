use std::io;

fn main() {
    println!("Digite a Altura");

    let mut altura = String::new();

    io::stdin()
        .read_line(&mut altura)
        .expect("Failed to read line");

    let altura1: i32 = converter_string_para_i32(altura);

    println!("Digite a Largura");

    let mut largura = String::new();

    io::stdin()
        .read_line(&mut largura)
        .expect("Failed to read line");

    let  largura1: i32 = converter_string_para_i32(largura);


    println!("Digite o comprimento");

    let mut comprimento = String::new();

    io::stdin()
        .read_line(&mut comprimento)
        .expect("Failed to read line");

    let  comprimento1: i32 = converter_string_para_i32(comprimento);
    
    if largura1 > altura1 {
        println!( "O volume da caixa é: {}", comprimento1 * largura1 * altura1 )        
    }else{
        print!("A largura tem que ser maior que a altura")
    }

    fn converter_string_para_i32(uma_string: String) -> i32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}