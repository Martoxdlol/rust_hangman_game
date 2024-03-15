use rand::prelude::*;
use std::fs;
use std::io;
use std::io::Write;

fn main() {
    loop {
        start_game();
    }
}

fn input(text: &str) -> Result<String, io::Error> {
    let mut input = String::new();
    print!("{}", text);
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input)?;
    return Ok(input.trim().to_string());
}

fn enforce_input(text: &str) -> String {
    loop {
        match input(text) {
            Ok(t) => {
                return t;
            }
            _ => {}
        }
    }
}

fn start_game() {
    clear_console();

    let target_word = obtain_target_word();

    let mut attempts: Vec<String> = Vec::new();

    let mut count: i32 = 8;

    let mut slots: String = generate_slots(&target_word, &attempts);

    loop {
        clear_console();

        println!("Bienvenido al ahorcado 😵");
        println!("Estado: {}", slots);
        println!("Adivinado hasta ahora: {}", attempts.join(", "));
        println!("Vidas: {}", count);

        if slots_fully_filled(&slots) {
            println!("¡Ganaste! (enter para continuar)");
            let _ = input("");
            break;
        }

        let attempt: String = enforce_input("Adivinar letra o palabra: ").to_ascii_lowercase();

        if attempts_has_word(&attempts, &attempt) {
            continue;
        }

        let mut next_slots = slots.clone();

        if attempt.len() > 1 {
            if target_word.eq(&attempt) {
                next_slots = attempt.clone();
            }
            attempts.push(attempt);
        } else {
            attempts.push(attempt);
            next_slots = generate_slots(&target_word, &attempts);
        }

        if next_slots.eq(&slots) {
            count -= 1;
        } else {
            slots = next_slots;
        }

        if slots_fully_filled(&slots) {
            continue;
        }

        if count == 0 {
            println!("¡Perdiste! (enter para continuar)");
            let _ = input("");
            break;
        }
    }
}

fn generate_slots(target_word: &String, attempts: &Vec<String>) -> String {
    let slots = target_word
        .chars()
        .map(|char| {
            if attempts_has(attempts, char) {
                char
            } else if char == ' ' {
                ' '
            } else {
                '_'
            }
        })
        .collect();
    slots
}

fn clear_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn attempts_has(attempts: &Vec<String>, char: char) -> bool {
    for attempt in attempts {
        match attempt.chars().next() {
            Some(c) => {
                if c == char {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn attempts_has_word(attempts: &Vec<String>, word: &String) -> bool {
    for attempt in attempts {
        if attempt.eq(word) {
            return true;
        }
    }
    false
}

fn slots_fully_filled(slots: &String) -> bool {
    for char in slots.chars() {
        if char == '_' {
            return false;
        }
    }

    true
}

fn obtain_target_word() -> String {
    let words = read_words_file("words.txt");

    match words {
        Some(w) => {
            let word = choose_random_word(&w);
            return word.to_ascii_lowercase();
        }
        _ => {
            println!("Para que la palabra se elija automáticamente, necesitas un archivo 'words.txt' en el directorio actual con al menos una palabra.");
            enforce_input("Ingrese una palabra (no se la muestres a los jugadores): ")
                .to_ascii_lowercase()
        }
    }
}

fn read_words_file(filename: &str) -> Option<Vec<String>> {
    let result = fs::read_to_string(filename);

    match result {
        Err(_) => {
            return None;
        }
        Ok(content) => {
            let words: Vec<String> = content
                .lines()
                .map(|line| line.to_string())
                .filter(|line| line.len() > 0)
                .collect();

            if words.len() == 0 {
                return None;
            }

            return Some(words);
        }
    }
}

fn choose_random_word(words: &Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let index: usize = rng.gen_range(0..words.len());
    words[index].clone()
}
