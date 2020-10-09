///Escrever um programa que leia o nome de um aluno e as notas de três provas, P1, P2 e P3.
/// O aluno é aprovado se possuir média aritmética (MD) acima de 7,
/// estará em recuperação entre 5 e 7, e será reprovado abaixo de 5.
/// Imprimir todos os dados capturados, e a situação do aluno.
///
/// 
/// Nao consigo passar o nome quando a nota é um numero real 2.5 (f32)
use std::io;

fn main() {
    println!("Digite seu nome");

    let mut nome = String::new();
    io::stdin()
        .read_line(&mut nome)
        .expect("Failed to read line");

    println!("Digite a  nota da primeiro prova ");

    let mut prova1 = String::new();
    io::stdin()
        .read_line(&mut prova1)
        .expect("Failed to read line");

    let p1: f32 = converter_string_para_f32(prova1);

    println!("Digite a  nota da segunda prova ");

    let mut prova2 = String::new();
    io::stdin()
        .read_line(&mut prova2)
        .expect("Failed to read line");

    let p2: f32 = converter_string_para_f32(prova2);

    println!("Digite a  nota da terceira prova ");

    let mut prova3 = String::new();
    io::stdin()
        .read_line(&mut prova3)
        .expect("Failed to read line");

    let p3: f32 = converter_string_para_f32(prova3);

    let media = (p1 + p2 + p3) / 3.0;

    if media >= 7.0 {
        println!(" {} Você esta aprovado! Sua média foi {}", media, nome);
    }
    if media >= 5.0 && media < 7.0 {
        println!(
            " Você esta de recuperacao! Sua média foi {} {}", media, nome);
    }
    if media < 5.0 {
        println!(" Você não passou! Sua média foi {} {}", media, nome);
    }

    fn converter_string_para_f32(uma_string: String) -> f32 {
        uma_string
            .trim()
            .parse()
            .expect("Falha ao converter o valor")
    }
}