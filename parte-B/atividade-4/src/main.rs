//Elaborar um programa que apresente os resultados das potências do valor de base 3, elevando a um expoente que varie do valor 0 até o valor 15.

//Restrição: Não utilize a função de exponenciação da biblioteca i32.
// a Nat me ajudou

fn main() {
    
let  base = 3;
//let  expoente = 0;
let  mut contador = 0;
let mut potencia: i32 ; 


   while contador <= 15{
       potencia = base * contador;
       
        println!(" {} ", potencia );
        contador += 1 ;
    }
}