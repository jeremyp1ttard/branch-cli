use std::process::Command;
use std::env;
use std::io;

use inflector::cases::pascalcase::to_pascal_case;


fn main() {
    let ticket_number = input_ticket_number();
    let ticket_description = input_description();

    let branch_name = format!("{}_{}", ticket_number.trim().to_uppercase(), to_pascal_case(ticket_description.trim()));

    match checkout_new_branch(&branch_name) {
        Ok(_) => println!("Checked out new branch: {}", branch_name),
        Err(error) => println!("Failed to checkout new branch: {}. /nError: {}", branch_name, error),
    }
}

fn checkout_new_branch(branch_name: &str) -> Result<(), std::io::Error> {
    let current_dir = env::current_dir().expect("Could not get current directory.");

    let output = Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(branch_name)
        .current_dir(&current_dir)
        .output()?;

    if output.status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to execute Git command"))
    }
}

fn input_ticket_number() -> String {
    let mut ticket_number = String::new();
    loop {
        println!("What is the ticket number?");
        io::stdin().read_line(&mut ticket_number).expect("Failed to read ticket number");

        if ticket_number.trim().to_lowercase() == "exit" {
            std::process::exit(0); // Exit the program if user enters 'exit'
        }
        
        if ticket_number.trim().chars().all(|c| c.is_alphanumeric() || c == '-') {
            break;
        } else {
            println!("Invalid ticket number, only letters, numbers, and hyphens are allowed");
        }
    }
 return ticket_number;
}

fn input_description() -> String {
    println!("Enter a brief descriion of the ticket");
    let mut ticket_description = String::new();
    io::stdin().read_line(&mut ticket_description).expect("Failed to read line");
    if ticket_description.trim().to_lowercase() == "exit" {
        std::process::exit(0); // Exit the program if user enters 'exit'
    }

    return ticket_description;
}