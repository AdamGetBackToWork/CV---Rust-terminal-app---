mod experience;
mod contact;
mod skills;

use inquire::Select;
use std::fs;
use colored::Colorize;
use contact::show_contact;
use skills::show_skills;
use experience::show_experience;

fn main() {
    println!("");
    println!("");
    println!("Hello fellow traveller! I'm {}, a fellow student at Warsaw Univeristy of Technology, seeking, discovering and learning new technologies","Adam Szajgin".bold().bright_yellow());

    let options = vec!["About","Experience","Skillset","Contact","Exit"];

    loop{
        let choice = Select::new("What would you like to know?", options.clone()).prompt();

        match choice {
            Ok(choice) => {
                if choice == options[0] {
                    println!("");
                    println!("Boi did you choose a hard one... *About me*?! What am I supposed to say...  ");
                    println!("Anyway, I am the most driven folk you will ever meet, I am passionate about everything science-related, and always seeking knowledge.");
                    println!("In terms of *How am I at use to you?* Well, I know how to do plenty of stuff, e.g. {}","C/C++ development, Python @ AI/ML, SQL and so much more!".bold().bright_green());
                    println!("But not only that, i also know how to talk to people, and I mean REALLY..., I've worked with people for 3 years at project management as a PM and coordinator");
                    println!("So not only do I know how to write a bit of code, but also I'm a pleasure to work with and can lead the team to greatness!");
                    println!("");
                    println!("Even though i like talking to people i hate talking about myself, well in this case write...");
                    println!("");
                }
                else if choice == options[1] {
                    let file_path = "./data/experience/experience.json".to_owned();
                    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_experience(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in experience.rs"),
                    }
                }
                else if choice == options[2] {
                    let file_path = "./data/skills/skills.json".to_owned();
                    let contents = fs::read_to_string(file_path).expect("Couldn't find or load that file.");
                    let res = show_skills(&contents);
                    match res {
                        Ok(_res) => println!(""),
                        Err(_) => println!("Error in experience.rs"),
                    }
                }
                else if choice == options[3] {
                    show_contact();
                }
                else if choice == options[4] {
                    println!("Cy@! Hope you enjoyed the ride!");
                    break;
                }
            },
            Err(_) => println!("Not a valid option selected"),
        }
    }
}
