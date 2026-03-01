
// Commented out other assignments as Rust only runs one main file. Can uncomment other files and keep one uncommented.

// =================================================
// Assignment 1: Temperature Converter
// =================================================
// Create a Rust program that converts temperatures between Fahrenheit and Celsius.
 // The program should: 

// Declare a constant for the freezing point of water in Fahrenheit (32°F).
const FREEZING_POINT_F: f64 = 32.0;

/* Implement two functions: 
fahrenheit_to_celsius(f: f64) -> f64: Converts Fahrenheit to Celsius
*/
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}


// celsius_to_fahrenheit(c: f64) -> f64: Converts Celsius to Fahrenheit 
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}
/* In the main function:
Declare a mutable variable with a temperature in Fahrenheit
Convert it to Celsius using your function and print the result
Use a loop to convert and print the next 5 integer temperatures (e.g., if you start with 32°F, print conversions for 33°F, 34°F, 35°F, 36°F, and 37°F) */
fn main() {
    println!(" \n Assignment 1 : Temperature Converter \n");
    let mut temperature_f: f64 = FREEZING_POINT_F;

    let temperature_c = fahrenheit_to_celsius(temperature_f);
    println!("{:.2} F = {:.2} C", temperature_f, temperature_c);

    for _ in 0..5 {
        temperature_f += 1.0;
        let temperature_c = fahrenheit_to_celsius(temperature_f);
        println!("{:.2} F = {:.2} C", temperature_f, temperature_c);
    }
    // wanted them in two decimal places
}


// =================================================
// ASSIGNMENT 2 HERE (uncomment to see Assignment 2)
// =================================================

/* 
// Assignment 2: Number Analyzer 
fn main() {
// 1. Create an array of 10 integer numbers of your choice.
let numbers = [5, 10, 15, 20, 25, 30, 35, 40, 45, 50];
// For this one i decided to use by five...

/* 2. Implement a function is_even(n: i32) -> bool that returns true if a number is even,
 false otherwise.
 */
fn is_even(n: i32) -> bool {
    n % 2 == 0
}
println! ("Number Analysis:");

// 3. Use a for loop to iterate through the array and for each number:
    // Print whether it's even or odd using your is_even function
    //If the number is divisible by 3, print "Fizz" instead
    // If the number is divisible by 5, print "Buzz" instead
    // If it's divisible by both 3 and 5, print "FizzBuzz"

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        } else if num % 3 == 0 {
            println!("{}: Fizz", num);
        } else if num % 5 == 0 {
            println! ("{}: Buzz", num);
        } else if is_even(num) {
            println!("{}: Even", num);
        } else {
            println!("{}: Odd", num);
        }
    }
// 4. Use a while loop to find and print the sum of all numbers in the array.
    let mut sum = 0;
    let mut i = 0;
    while i < numbers.len() {
        sum += numbers[i];
        i += 1;
    } 
    println!("\n Sum of numbers: {}", sum);
// 5. Use a loop to find and print the largest number in the array.
    let mut largest = numbers[0];
    for &num in numbers.iter() {
        if num > largest {
            largest = num;
        }
    }
    println!("Largest number: {}", largest);
}
*/

/* 
// =================================================
// ASSIGNMENT 3
// =================================================
/* Assignment 3: Guessing Game
Create a simple number guessing game in Rust. The program should:

Use a mutable variable to store a "secret" number (you can hard-code this).
Implement a function check_guess(guess: i32, secret: i32) -> i32 that returns:
0 if the guess is correct
1 if the guess is too high
-1 if the guess is too low
In the main function:
Use a loop to repeatedly:
Set a mutable guess variable to a number of your choice (simulating user input)
*/
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main(){
    let secret_number: i32 = 9; //Says we can pick it hard-code a num
    let mut guess_count: i32 = 0;
    let mut guess: i32 = 0;

/* Call the check_guess function
Use an if-else expression to print whether the guess was correct, too high, or too low
If the guess was correct, break the loop
After the loop ends, print how many guesses it took (you'll need to track this in a variable)*/
    loop {
        guess_count += 1;

        if guess_count == 1 {
            guess = 3;
        } else if guess_count == 2 {
            guess = 11;
        } else if guess_count == 3 {
            guess = 9; //This one is correct :D
        }
        let result = check_guess(guess, secret_number);
        if result ==0 {
            println!("Correct! Secret number was {}...", secret_number);
            break;
        } else if result == 1 {
            println!("Too high!");
        } else {
            println!("Too low!");
        }
    }
    println!("It took {} guesses.", guess_count);
}

*/