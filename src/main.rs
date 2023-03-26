use std::io;


fn create_secret_word_vector(word: &String) -> Vec<char> {
    let mut word_vector = Vec::new();
    for c in word.chars() {
        word_vector.push(c.to_ascii_lowercase());
    }
    word_vector
}

fn create_hidden_vector (secret_vector: &Vec<char>) -> Vec<char> {
    let lenght = secret_vector.len();
    let mut new_vec = Vec::new();
    for i in 0..lenght {
        new_vec.push('_');
    };
    new_vec
}

fn print_stickman(lives: &u32){
    match lives {
        5 => {
            println!("______");
            println!("|     ");
            println!("|     ");
            println!("|     ");
            println!("|     ");
        },
        4 => {
            println!("______");
            println!("|    o");
            println!("|     ");
            println!("|     ");
            println!("|     ");
        },
        3 => {
            println!("______");
            println!("|    o");
            println!("|    |");
            println!("|     ");
            println!("|     ");
        },
        2 => {
            println!("______");
            println!("|    o");
            println!("|   /|\\");
            println!("|     ");
            println!("|     ");
        },
        1 => {
            println!("______");
            println!("|    o");
            println!("|   /|\\");
            println!("|   / ");
            println!("|     ");
        },
        0=> {
            println!("______");
            println!("|    o");
            println!("|   /|\\");
            println!("|   / \\");
            println!("|       ");
        },
        _ => {},
    }
}

fn main() {
    let mut lives = 5;
    let secret_word = "Programming";
    let secret_word: String = secret_word.to_ascii_lowercase();
    let mut secret_word_vec = create_secret_word_vector(&secret_word);
    let mut hidden_vector = create_hidden_vector(&secret_word_vec);
    let mut used_letters = Vec::<char>::new();
    let mut hidden_string: String = hidden_vector.iter().collect();
    while &lives > &0 {
        print_stickman(&lives);
        hidden_string = hidden_vector.iter().collect();
        
        println!("Word: {}, remaining lives: {}", hidden_string, lives);
        if !hidden_string.contains("_"){
            break
        }
        println!("Please input a character: ");
        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).expect("failed to read the line");
        match &user_input.len() {
            0|1 => {
                println!("You entered an empty string ");
                continue
            },
            2 => println!("Your character is {}", user_input),
            _ => {
                println!("You enter too many characters");
                continue
            }
        }
        let user_input = user_input.chars().next().unwrap();
        if secret_word.contains(user_input) && hidden_vector.contains(&user_input){
            println!("You already found that letter");
            continue
        } else if secret_word.contains(user_input) && !hidden_vector.contains(&user_input) {
            println!("Found a new letter!");
            for i in 0..secret_word_vec.len() {
                if secret_word_vec[i] == user_input{
                    secret_word_vec[i] = '_';
                    hidden_vector[i] = user_input;
                    used_letters.push(user_input);
                    
                } 
            }
        } else if !secret_word.contains(user_input) && !used_letters.contains(&user_input) {
            used_letters.push(user_input);
            lives -= 1;
            println!("No letter found, your remaining lives are {}", lives);
        } else if used_letters.contains(&user_input) {
            println!("You already tried that letter");
        }

    }


    if lives == 0 {
        print_stickman(&lives);
        println!("Game over, you lose");
    } else {
        println!("You win!");
    };
    


}
