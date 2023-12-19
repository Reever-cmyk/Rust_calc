use std::io;
fn main() {
    println!("Hallo ich bin dein interaktiver Taschenrechner.");
    loop {
        println!("Input:");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Eingabefehler");

        let result = calc(input.trim());
        match result {
            Some(res) => println!("Das Ergebnis: {:?}", res),
            None => println!("Ungültige Eingabe!"),
        }
    }
}

fn calc(input: &str) -> Option<Result<f64, &str>> {
    let mut nums: Vec<f64> = Vec::new();
    let mut ops: Vec<char> = Vec::new();
    let mut num_buffer = String::new();

    for ch in input.chars() {
        match ch {
            '*' | '/' | '+' | '-' => {
                if num_buffer.is_empty() {
                    return Some(Err("Ungültige Eingabe: Operator ohne Operanden"));
                }
                let num = num_buffer.parse::<f64>().ok()?;
                nums.push(num);
                num_buffer.clear();
                ops.push(ch);
            }
            '0'..='9' | '.' => {
                num_buffer.push(ch);
            }
            _ => return Some(Err("Ungültige Eingabe: Ungültiges Zeichen")),
        }
    }

    if !num_buffer.is_empty() {
        let num = num_buffer.parse::<f64>().ok()?;
        nums.push(num);
    }

    if nums.len() != ops.len() + 1 {
        return Some(Err("Ungültige Eingabe: Nicht genug Operatoren oder Operanden"));
    }

    // Berechne zuerst Multiplikationen und Divisionen
    let mut i = 0;
    while i < ops.len() {
        if ops[i] == '*' || ops[i] == '/' {
            let result = match ops[i] {
                '*' => nums[i] * nums[i + 1],
                '/' => {
                    if nums[i + 1] != 0.0 {
                        nums[i] / nums[i + 1]
                    } else {
                        return Some(Err("Ungültige Eingabe: Division durch Null"));
                    }
                }
                _ => unreachable!(), // Unmöglicher Fall
            };
            nums[i] = result;
            nums.remove(i + 1);
            ops.remove(i);
        } else {
            i += 1;
        }
    }

    // Berechne Additionen und Subtraktionen
    let mut result = nums[0];
    for i in 0..ops.len() {
        match ops[i] {
            '+' => result += nums[i + 1],
            '-' => result -= nums[i + 1],
            _ => return Some(Err("Ungültige Eingabe: Ungültiger Operator")), // Ungültiger Operator
        }
    }

    Some(Ok(result))
}
