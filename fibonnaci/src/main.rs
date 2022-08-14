fn next_fibonnaci(current_value: u64, previous_value: u64) -> u64 {
    current_value + previous_value
}

use std::io;

fn main() {
    let mut user_input = String::new();
    
    let mut previous_value: u64 = 1;
    let mut current_value: u64 = 1;
    let mut next_value: u64 = 0;
    
    println!("Os primeiros dois valores da sequência de Fibonnaci são: {} {}", previous_value, current_value);
    loop {
        
        
        println!("Caso deseje parar de calcular a sequência de Fibonnaci, digite 'P'.");
        println!("Você deseja continuar calculando a sequência?");

        io::stdin().read_line(&mut user_input).expect("Falha ao ler a linha!");

        if user_input == "P" {
            println!("Parando o programa...");
            break;
        }
        next_value = next_fibonnaci(previous_value, current_value);

        println!("O próximo valor da sequência de Fibonnaci é: {}", next_value);
        previous_value = current_value;
        current_value = next_value;
    }
}
