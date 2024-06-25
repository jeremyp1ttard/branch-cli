use std::process::Command;
use std::env;
use std::io;

use inflector::cases::pascalcase::to_pascal_case;


fn main() {
    let ticket_number = input_ticket_number();
    let ticket_description = input_description();

    let branch_name = format!("{}_{}", ticket_number.trim().to_uppercase(), to_pascal_case(ticket_description.trim()));

    checkout_new_branch(branch_name);
}

fn checkout_new_branch(branch_name: String) {
    let current_dir = env::current_dir().expect("could not get current directory.");

    let output = Command::new("git").arg("checkout").arg("-b").arg(&branch_name).current_dir(&current_dir).output().expect("failed to execute process");
    
    if output.status.success() {
        println!("Checked out new branch: {}", branch_name);
    } else {
        println!("Failed to checkout new branch: {}", branch_name);
    }

}

fn input_ticket_number() -> String {
    let mut ticket_number = String::new();
    loop {
        println!("What is the ticket number?");
        io::stdin().read_line(&mut ticket_number).expect("Failed to read ticket number");
        
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

    return ticket_description;
}