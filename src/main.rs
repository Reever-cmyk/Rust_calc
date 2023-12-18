// Taschenrechner Projekt. 1. Grundlegende Operationen erkennen +, -, *, /,...
use std::io;
fn main() {
    println!("Hallo ich bin dein interaktiver Taschenrechner.");
    loop{
        println!("Wie kann ich dir behilflich sein? Gib deine Mathe Problem direkt ein.");

        let mut input = String::new();
        let sign = ['+','-','*','/'];
        io::stdin().read_line(&mut input).expect("Das habe ich nicht verstanden bitte nocheimal.");

        if let Some(position) = input.find(sign) {
            let operator = input.chars().nth(position).unwrap();
            println!("Der Operator ist: {}", operator);
        } else {
            println!("Kein Operator gefunden.");
        }


        println!("inputstring {input}");
        println!("Operator {operator}");

    }
}
