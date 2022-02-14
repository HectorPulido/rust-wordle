use rand::Rng;
use std::fs;

const FILENAME: &str = "animals.txt";
const FAIL: &str = "⬜";
const SUCCESS: &str = "🟩";
const MISPLACE: &str = "🟨";

const ATTEMPTS: i8 = 6;

fn get_wordle(attemp: String, correct: &String) -> (String, bool) {
    let mut result = "".to_string();
    let mut is_correct = true;

    let attemp_chars: Vec<char> = attemp.chars().collect();
    let correct_chars: Vec<char> = correct.chars().collect(); //.nth(0).unwrap();

    for (i, c) in correct_chars.iter().enumerate() {
        if let Some(attemp_char) = attemp_chars.get(i) {
            if c == attemp_char {
                result = format!("{}{}", result, SUCCESS);
                continue;
            }
        }

        is_correct = false;

        if attemp.contains(*c) {
            result = format!("{}{}", result, MISPLACE);
            continue;
        }

        result = format!("{}{}", result, FAIL);
    }

    (result, is_correct)
}

fn main() {
    // Config
    let animals = fs::read_to_string(FILENAME).expect("Something went wrong reading the file");
    let animals = animals.to_lowercase();
    let split = animals.split("\n");
    let animals_words = split.collect::<Vec<&str>>();

    let selected_index = rand::thread_rng().gen_range(0..animals_words.len());
    let selected_word = animals_words[selected_index]
        .trim()
        .to_lowercase()
        .to_string();

    let mut wordles: Vec<String> = vec![];

    println!("{}", selected_word);

    // Gameloop
    for i in 0..ATTEMPTS {
        println!("Por favor ingresa tu animal {}/{}:", i, ATTEMPTS);
        let mut attempt = String::new();
        std::io::stdin().read_line(&mut attempt).unwrap();
        let attempt = attempt.to_lowercase().trim().to_string();

        let (wordle, correct) = get_wordle(attempt, &selected_word);

        println!("Rustle (ES) {}/{}:", i, ATTEMPTS);
        println!("{}", wordle);
        wordles.push(wordle);

        if correct {
            println!("¡Felicitaciones!");
            println!("Rustle (ES) {}/{}:", i, ATTEMPTS);
            break;
        }
    }

    // Game end
    for wordle in wordles {
        println!("{}", wordle);
    }
}
