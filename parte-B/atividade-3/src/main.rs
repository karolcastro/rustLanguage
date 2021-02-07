//Construir um programa que apresente todos os valores numéricos divisíveis por 4 e menores que 200.
//Restrição: Utilize o laço condicional pós-teste.


fn main() {
   let  mut numero = 0;
   
    loop {

        if numero >= 200{
            break ;
        }
        println!(" {} ", numero);
        numero +=4;
    }
}