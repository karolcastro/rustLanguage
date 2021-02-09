////A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.
/// 
/// Elaborar um programa que leia 10 elementos (valores reais) para temperatura em graus Celsius e armazene esses valores em um vetor A. O programa ao final deve apresentar a menor, a maior e a média das temperaturas lidas.


fn main() {

    let temp1 = &[34.5, 32.5, 28.3, 25.4, 30.0, 31.0, 33.3, 26.6, 27.6, 27.8];
    let  mut contador :f32 = 0.0;

    let mut menor_temperatura :f32 = temp1[0];
    let mut maior_temperatura :f32 = temp1[0];
    
   
    //calcular menor
    for i in temp1.iter(){
        if i < &menor_temperatura {
            menor_temperatura = *i;
        } 
    }

    
    for i in temp1.iter(){
        if i > &maior_temperatura {
            maior_temperatura = *i;
        } 
    }
   
    for i in temp1.iter(){
        contador = contador + i;
    }
    let media_temperatura :f32 = contador / 10.0;

    //Ler todas as temperaturas
    println!("A menor temperatura é = {}, A maior é = {} e a média é = {}.", menor_temperatura,maior_temperatura, media_temperatura);
    
    
}

// fn converter_string_para_i32(uma_string: String) -> i32 {
//     uma_string
//         .trim()
//         .parse()
//         .expect("Falha ao converter o valor")
// }
