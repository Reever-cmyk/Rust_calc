//TODO: fn main() -> allg. Ablauf, read_input
// tokenize -> input wird in tokens aufgeteilt und gespeichert
// Enumtyp -> basierend auf Operanden, Klammer, Potenz, ... Enum erstellen.
// fn evaluate() -> berechnet gesamte aufgabe mittels token
use std::io;
fn main(){
    println!("TR start...");
    loop{
        println!("Rechnung: ");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Eingabefehler");
        let input_trimmed = input.trim();
        let output = tokenize(&input_trimmed);
        println!("tokens: {:?}", output);

        let result = evaluate(tokenize(&input_trimmed));
        println!("Ergebniss: {:?}", result);


    }
}

#[derive(Debug)]
enum Token{
    Number(f64),
    Operator(char),
    Power(char),
    LeftPara(char),
    RightPara(char),
    EOF,

}

fn tokenize(input: &str) -> Vec<Token>{
    let mut tokens = Vec::new();
    for ch in input.chars(){
        match ch{
            '+'|'-'|'*'|'/' => {
                tokens.push(Token::Operator(ch));
            }
            '0'..='9' =>{
                let num  = ch.to_digit(10).unwrap() as f64;
                tokens.push(Token::Number(num));
            }
            '^' => {
                tokens.push(Token::Power(ch));
            }
            '(' =>{
                tokens.push(Token::LeftPara(ch));
            }
            ')' =>{
                tokens.push(Token::RightPara(ch));
            }
            _=> {
                println!("Unbekanntes Zeichen: {}", ch);
            }
        }
    }
    tokens.push(Token::EOF); // End of File
    tokens
}

fn evaluate(tokens: Vec<Token>) -> Option<f64> {
    let mut op_stack: Vec<char> = Vec::new();
    let mut num_stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Number(num) => num_stack.push(num),
            Token::Operator(op) => {
                while !op_stack.is_empty()
                    && precedence(&op) <= precedence(&op_stack[op_stack.len() - 1])
                {
                    perform_operation(&mut num_stack, &mut op_stack);
                }
                op_stack.push(op);
            }
            _ => return None, // Unbekannter Token
        }
    }

    while !op_stack.is_empty() {
        perform_operation(&mut num_stack, &mut op_stack);
    }

    if num_stack.len() == 1 {
        num_stack.pop()
    } else {
        None // Unerwartete Anzahl von Operanden übrig
    }
}


fn precedence(op: &char) -> usize {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => 0, // Unbekannter Operator
    }
}

fn perform_operation(num_stack: &mut Vec<f64>, op_stack: &mut Vec<char>) {
    if let Some(a) = num_stack.pop() {
        if let Some(b) = num_stack.pop() {
            let op = op_stack.pop().unwrap();
            match op {
                '+' => num_stack.push(b + a), // Die Reihenfolge der Operanden ist umgekehrt
                '-' => num_stack.push(b - a),
                '*' => num_stack.push(b * a),
                '/' => num_stack.push(b / a),
                _ => (), // Unbekannter Operator
            }
        }
    }
}

