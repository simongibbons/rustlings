// Fill in the rest of the line that has code missing!
// No hints, there's no tricks, just get used to typing these :)

fn main() {
    // Characters (`char`)

    let my_first_initial = 'C';
    print_character_class(my_first_initial);

    let your_character = '🤔';
    print_character_class(your_character);
}


fn print_character_class(character: char) {
    if character.is_alphabetic() {
        println!("Alphabetical!");
    } else if character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
}
