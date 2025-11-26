//First Rust mini_project by Alan Gomez

use std::io::{self, Write};

fn main() {
    loop {
        println!("Menu -- Select an option!!!!");
        println!("1. Converter to F");
        println!("2. Converter to C");

        let mut number_selected:String = String::new();// std::string number_selected {""};
        io::stdin().read_line(&mut number_selected).expect("Fail read");
        print!("You choiced > {:.2}", number_selected);

        // number_selected = String::from("TRESS");
        // println!("\t .3. : {}", number_selected);
        // println!("\t .3. : {:.4}", number_selected);
        // number_selected = String::from("sTRESS");
        // println!("\t .3. : {:.4}", &number_selected[1..5]);

        //let numero: i32 = number_selected.trim().parse().expect("Conversión fallida"); 
        // println!("Numero> {}", number_selected);
        // println!("Numero> {}", numero*2);
    
        if number_selected.trim() == "1"{
            converter_f();
        } else if number_selected.trim( ) == "2"{
            converter_c();
        }
    }
}

// Aquí se hace con match en vez de .exept para cuando hay error en conversion.
fn converter_f(){
    print!("Converter Celsius to Faraheit!! \n");
    print!("Insert amount> ");

    io::stdout().flush().unwrap(); // Esperar que el buffer se vacie.
    let mut conv : String = String::new();
    io::stdin().read_line(&mut conv).expect("Error de Ingreso");

    let fahariat: Result<f64, _> = conv.trim().parse();

    match fahariat{
        Ok(mut c) =>{
                c = (c * 9.0 / 5.0) + 32.0;
                println!("Conversion> {:.2}", c);
            },
        Err(_) =>{
                println!("Error en Parse");
            }
    }

    
}

fn converter_c(){
    print!("Converter  Faraheit to Celsius!! \n");
    print!("Insert amount> ");

    io::stdout().flush().unwrap(); // Esperar que el buffer se vacie.
    let mut conv : String = String::new();
    io::stdin().read_line(&mut conv).expect("Error de Ingreso");

    let mut celsius: f64 = conv.trim().parse().expect("Error Parse");

    celsius = (celsius - 32.0) * 5.0 / 9.0;
    println!("Conversion> {:.2}", celsius);

}

