//Name: Angelica Vega
//Using the sample code from the assignment along with adding the missing functions/code
use std::fs::File; //starter code from assignment.
use std::io::{Write, BufReader, BufRead};

struct Book {
    title: String,
    author: String,
    year: u16,
}

fn save_books(books: &Vec<Book>, filename: &str) {
    // TODO: Implement this function
    // Hint: Use File::create() and write!() macro
    // time to make a file...
    let mut file = File::create(filename)
    // now upwraps
        .expect("Failed to create a file");

    for book in books {
        writeln!(file, "{},{},{}", book.title, book.author, book.year)
            .expect("Failed to write into file");
    }
}

fn load_books(filename: &str) -> Vec<Book> {
    // TODO: Implement this function + opening file
    // Hint: Use File::open() and BufReader
    let file = File::open(filename)
        .expect("Failed to open file");
    let reader = BufReader::new(file); //wrapping the fike in BufReader
    let mut books: Vec<Book> = Vec::new(); //creating an empty 

    // each line file
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect(); 
        // upwraps and split line using the comma

        // lines will have three parts: title, author, year
        if parts.len() == 3 {
            let title = parts[0].to_string();
            let author = parts[1].to_string();
            let year: u16 = parts[2]
                .parse()
                .expect("Year must be a number"); 
            books.push(Book {title,author, year});
        }
    }
    books

}

fn main() {
    let books = vec![
        Book { title: "1984".to_string(), author: "George Orwell".to_string(), year: 1949 },
        Book { title: "To Kill a Mockingbird".to_string(), author: "Harper Lee".to_string(), year: 1960 },
    ];

    save_books(&books, "books.txt");
    println!("Books saved to file.");

    let loaded_books = load_books("books.txt");
    println!("Loaded books:");
    for book in loaded_books {
        println!("{} by {}, published in {}", book.title, book.author, book.year);
    }
}