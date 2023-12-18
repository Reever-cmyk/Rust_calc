// Taschenrechner Projekt. 1. Grundlegende Operationen einpflegen +, -, *, /,...
use std::io;

fn main() {

    loop{
        let mut cmd = String::new();

        println!("Hallo ich bin dein interaktiver Taschenrechner.");
        println!("Wie kann ich dir helfen? Du kannst deine Mathe Probleme direkt eingeben.");

        io::stdin().read_line(&mut cmd).expect("Das habe ich nicht verstanden bitte nocheinmal.");

        let parts: Vec<&str> = cmd.trim().split('+').collect();
        if parts.len() >= 2 {
            // Teile vor und nach Pluszeichen in Zahlen umwandeln
            if let (Ok(num1), Ok(num2)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                let summe = num1 + num2;
                println!("Die Summe von {} und {} ist: {}", num1, num2, summe);
            } else {
                println!("Fehler beim Umwandeln der Teile in Zahlen.");
            }
        } else {
            println!("Der eingegebene String enthält kein Pluszeichen oder hat nicht genügend Teile.");
        }
    }
}
