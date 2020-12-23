fn main() {

    let mut contador = 1;
    let mut resultado_fatorial = 1;

    while contador <= 5 {

        resultado_fatorial = resultado_fatorial * contador;
        contador = contador + 1;
    }
    
    println!("O resultado é {:?}", resultado_fatorial)

}
