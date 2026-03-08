// As instructed in the Assignment, create an interactive Rust Program
// That performs basic file operations by executing system commands using Command::new()
// In other words, needs to be interactive. 
use std::io::{self, Write};
use std::process::Command;
// Create an enum FileOperation with variants for each of these operations:
enum FileOperation {
    List(String),               // Directory path
    Display(String),            // File path
    Create(String, String),     // File path and content
    Remove(String),             // File path
    Pwd,                        // Print working directory
}

// 2. Implement User Input Loop with Menu
// 3. Implement the perform_operation Function

fn perform_operation(operation: FileOperation) {
    // Implement command execution based on the operation
    //also referencing from example in step 4.
    match operation {
        //List the files in a directory
        FileOperation::List(directory_path) => {
            Command::new("ls")
                .arg(directory_path)
                .status()
                .expect("Failed to execute ls");
        }
        // This will display the file contents
        FileOperation::Display(file_path) => {
            Command::new("cat")
                .arg(file_path)
                .status()
                .expect("Failed to execute cat");
        }
        
        //Create
        FileOperation::Create(file_path,content ) => {
            let command = format!("echo '{}' > {}", content, file_path);
            Command::new("sh")
                .arg("-c")
                .arg(command)
                .status()
                .expect("Failed to create file");

            print!("\nFile '{}' created successfully\n", file_path)
                
        }
        
        // Remove
        FileOperation::Remove(file_path) => {
            Command::new("rm")
                .arg(&file_path)
                .status()
                .expect("Failed to remove file");

            print!("\nFile '{}' removed successfully\n", file_path)
        }

        // Pwd
        FileOperation::Pwd => {
            Command::new("pwd")
            .status()
            .expect("Failed to execute pwd");
        }
    }
}

// Now after making the functions with the required list, display, create, remove and pwd,
// Continue with assignment ->
// 5. Implement Minimal Error Handling
// 6. Program Termination

// This is a helper function for the reader user input
fn read_input(prompt: &str) -> String {
    let mut input = String::new();

    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    input.trim().to_string()
}
// Implementing what is made for the assignment into main
fn main() {
    println!("Welcome to the File Operations Program!");

    loop {
        println!("\nFile Operations Menu:");
        println!("1. List files in a directory");
        println!("2. Display file contents");
        println!("3. Create a new file");
        println!("4. Remove a file");
        println!("5. Print working directory");
        println!("0. Exit");
        
        let choice = read_input("\nEnter your choice (0-5): ");
        match choice.as_str(){
            "1" => {
                let dir = read_input("Enter directory path: ");
                print!("\n");
                perform_operation(FileOperation::List(dir));
            }

            // If the option is 2
            "2" => {
                let file = read_input("Enter file path: ");
                print!("\n");
                perform_operation(FileOperation::Display(file));

            }

            "3" => {
                let file = read_input("Enter file path: ");
                let content = read_input("Enter content: ");
                perform_operation(FileOperation::Create(file,content));
                
            }

            // If choice 4 is picked
            "4" => {
                let file = read_input("Enter file path: ");
                perform_operation(FileOperation::Remove(file));
            }

            "5" => {
                println!("\nCurrent working directory:");
                perform_operation(FileOperation::Pwd);
            }

            "0" => {
                println!("\nGoodbye!");
                break;
            }
            // _ actually focuses on (any other) which will tell the user it is invalid
            _ => {
                println!("Invalid menu option. Please try again.");
            }
        }

    }
}