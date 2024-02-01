use std::io;
use std::io::{Read, Write};

fn main() {
    let x = true;
    let y = false;

    println!("O valor lógico verdadeiro de x ...: {}", x);
    println!("O valor lógico verdadeiro de y ...: {}", y);

    println!();
    print!("Tecle <Enter> para encerrar...");
    io::stdout().flush().unwrap();
    io::stdin().read(&mut [0u8]).unwrap();
}
