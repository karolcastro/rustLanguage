//Efetuar a leitura de um valor numérico inteiro que esteja na faixa de 1 até 9. 
//O programa deverá apresentar uma mensagem "O valor está na faixa permitida", caso o valor informado esteja na faixa. 
//Se o valor estiver fora da faixa permitida, deverá apresentar a mensagem "O valor está fora da faixa permitida".



fn main() {

    let number = 9;

    if number >= 1 && number <= 9{
        println!("O valor {} está na faixa permitida",number );
    }
    else {
        println!("O valor {} está fora da faixa permitida", number);
    }

    
}
