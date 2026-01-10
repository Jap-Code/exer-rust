/* 
Using a hash map and vectors, create a text interface to allow a user to
add employee names to a department in a company; for example, "Add
Sally to Engineering" or "Add Amir to Sales." Then, let the user retrieve
a list of all people in a department or all people in the company by
department, sorted alphabetically.
*/
use std::{collections::HashMap, io};

#[derive(Debug, Clone)]
enum Command {
    Add{ name: String, dept: Department },
    List{ dept: Department },
    All,
    Quit,
}

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Department {
    Finance,
    Sales,
    AD, 
    People,
}

// struct Employee {
//     name: String,
//     surname: String,
//     age: u8,
// }

fn get_input() -> String {
    println!("What would you like to do today? ");

    let mut raw_input = String::new();

    io::stdin()
        .read_line(&mut raw_input)
        .expect("Error while reading input ..");

    raw_input
}

fn extract_command(raw_input: &str) -> Option<Command> {

    let vect: Vec<&str> = raw_input
        .split_whitespace()
        .collect();

    match vect.as_slice() {
        ["Quit"]    => Some(Command::Quit),

        ["All"]     => Some(Command::All),

        ["List", dept]    => {
            let dept = parse_department(dept)?;
            Some(Command::List { dept })
        },

        ["Add", name, "to", dept] => {
            let dept = parse_department(dept)?;
            Some(Command::Add {
                name: name.to_string(),
                dept,
            })
        },

        _           => {
            eprintln!("Unknown command");
            return None
        },
    }
}


fn parse_department(word: &str) -> Option<Department> {
    match word {
        "Finance"   => Some(Department::Finance),
        "Sales"     => Some(Department::Sales),
        "AD"        => Some(Department::AD),
        "People"    => Some(Department::People),
        _ => None,
    }
}

fn execute_command(command: &Command, map: &mut HashMap<Department, Vec<String>>) {
    match command {
        Command::Quit       => {
            println!("Quitting..");
            std::process::exit(0);
        }
        Command::All        => {
            println!("Listing all Data from HashMap: ");
            list_all(map);
        }
        Command::List{ dept} => {
            println!("Listing people from {:?}", dept);
            list_data(&dept, map);
        }
        Command::Add{ name, dept} => {
            println!("Adding {} to {:?}", &name, &dept);    
            add_people(&name, &dept, map);
        }
    }
}

fn list_all(map: &mut HashMap<Department, Vec<String>>) {
    
    for (dept, people) in map {
        let mut sorted = people.clone();
        sorted.sort();
        println!("{:?}: {}", dept, sorted.join(", "));
    } 
}


fn list_data(dept: &Department, map: &mut HashMap<Department, Vec<String>>) {
    
    if let Some(people) = map.get(dept) {
        let mut cloned_people = people.clone();
        cloned_people.sort();
        println!("{:?}: {:?}", dept, cloned_people);
    } else {
        println!("No employees there yet!");
    };

}

fn add_people(name: &str, dept: &Department, map: &mut HashMap<Department, Vec<String>>) {
    
    map.entry(dept.clone()).or_insert(Vec::new()).push(name.to_string());
  
}

fn main() {
    
    let mut map: HashMap<Department, Vec<String>> = HashMap::new();
    
    loop {
        let raw_input = get_input();
        let command = extract_command(&raw_input);


        match command {
            Some(command)   => execute_command(&command, &mut map),
            None                     => eprintln!("No valid command found"),
        }
    }
}