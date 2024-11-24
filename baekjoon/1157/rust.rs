use std::io;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let words = input.trim().to_uppercase();
    
    let mut word_count = HashMap::new();
    for char in words.chars() {
        *word_count.entry(char).or_insert(0) += 1;
    }

    let mut sorted_words: Vec<(char, u32)> = word_count.into_iter().collect();
    sorted_words.sort_by(|a, b| b.1.cmp(&a.1));

    if sorted_words.len() > 1 && sorted_words[0].1 == sorted_words[1].1 {
        println!("?");
    } else {
        println!("{}", sorted_words[0].0);
    }
}
