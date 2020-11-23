//A partir do exercício 5, os próximos deverão utilizar vetores para armazenar os valores lidos.

//Elaborar um programa para calcular o máximo divisor comum de dois números inteiros positivos, MDC(x, y), usando o algoritmo de Euclides. Este algoritmo é baseado no fato de que se o resto da divisão de x por y representado por r, for igual a zero, y é o MDC. Se o resto r for diferente de zero, o MDC de x e y é igual ao MDC de y e r. O processo se repete até que o valor do resto da divisão seja zero.

fn main() {
    let MDC = [2, 4];
    let r = MDC[0] % MDC[1];
    

    loop {
         if r == 0 {
             break;
         };
         println!("{}", MDC[1]);
          
         if r != 0{
             break;
         }
         println!("{} {}", MDC[1],MDC[r])
    }
}
