use crate::ascii::*;

pub fn print_characters(word: &[&[u8]]) {
    let width = word.len() * ASCII_WIDTH as usize;

    for y in 0..ASCII_HEIGHT {
        for x in 0..width {
            // Check if we are in a space spot
            if x % ASCII_WIDTH as usize == 0 {
                print!("  ");
            }

            let character = word[x / ASCII_WIDTH as usize];

            if y as usize >= character.len() {
                continue;
            }
            
            let character_row = character[y as usize];
            let state = (character_row >> (ASCII_WIDTH - 1 - x as u32 % ASCII_WIDTH)) & 1;
            if state == 0 {
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!();
    }
}
