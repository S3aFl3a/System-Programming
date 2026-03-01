/* Assignment 2: Word Frequency Counter
Problem Statement
Create a program that:

Takes a string of text as input
Splits the text into words (space as separator) // text.split_whitespace().collect();
Counts the frequency of each word
Returns the word with the highest frequency and its count
Requirements:

Use mutable references where appropriate
Avoid using HashMaps or complex data structures
Solution
*/
// using this section from the assignment solution requirements-
//for this part, split
fn most_frequent_word(text: &str) -> (String, usize) {
    // Splitting the text to work 
    let words: Vec<&str> = text.split_whitespace().collect();
    //(max_word, max_count) // return tuple
    let mut max_word = "";
    let mut max_count = 0;

    // Looping and indexing here to avoid using HashMap
    for i in 0..words.len() {
        let current_word = words[i];
        let mut count = 0; //Works as a counter for frequency

        // Counting the amount of times the current_word appears here
        for x in 0..words.len() {
            if words[x] == current_word {
                count += 1;
            }
        }
        //Updating the maximum value if a high frequency is found here 
        //if count is more than the max count...
        if count > max_count {
            max_count = count;
            max_word = current_word;
        }
    }
    // Return tuple
    (max_word.to_string(), max_count)
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}

