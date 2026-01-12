use std::io;
use std::collections::HashMap;


#[derive(Debug, Clone)]
enum Command { 
    Add{ employee: Employee, department: Department},
    List{ department: Department},
    All,
    Quit,
}


#[derive(Debug, Clone, Eq, Hash, PartialEq)]
enum Department {
    AD, 
    Finance, 
    Marketing, 
    Sales, 
}


impl Department {
    fn choose_dept() -> Department {
        println!(
"Choose department: 
1) AD
2) Finance
3) Marketing
4) Sales");
        loop {
            match read_line()
                .trim()
                .parse() {
                    Ok(num) => match num {
                            1 => return Department::AD,
                            2 => return Department::Finance,
                            3 => return Department::Marketing,
                            4 => return Department::Sales,
                            _ => {
                                eprintln!("Invalid choice. Try again!");
                                continue
                            }
                        },
                    Err(_) => {
                        eprintln!("Invalid choice. Try again!");
                        continue  
                    }
                }
        }
    }
}


#[derive(Debug, Clone)]
struct Employee {
    name: String,
    surname: String, 
    age: u8,
}

impl Employee {
    fn new(name: String, surname: String, age: u8) -> Employee {
        Employee {
            name, 
            surname, 
            age,
        }
    }

}


fn empl_from_input() -> Employee {
    println!("Creating new employee..");
    
    println!("Name: ");
    let name = read_line();

    println!("Surname: ");
    let surname = read_line();

    println!("Age: ");
    let age: u8 = match read_line()
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_)  => {
                eprintln!("Invalid input: Defaulting to '30'!");
                30
            },
        };

    Employee::new(
        name, 
        surname, 
        age,
    )
}


fn read_line() -> String {
    let mut var = String::new();
    io::stdin()
        .read_line(&mut var)
        .expect("Error reading input!");

    if var.ends_with('\n') || var.ends_with('\r') {
        var.pop();
    }

    var
}


fn operations(num: u32) -> Command {
    match num {
        1 => Command::Add{ employee: empl_from_input(), department: Department::choose_dept() },
        2 => Command::List{ department: Department::choose_dept() },
        3 => Command::All,
        _ => Command::Quit,
    }
}


fn list(comp: &HashMap<Department, Vec<Employee>>) {
    println!("{:#?}", comp);
}


fn list_dept(comp: &HashMap<Department, Vec<Employee>>, dept: &Department) {
    
    if let Some(people) = comp.get(dept) {
        println!("{:?}: {:?}", dept, people)
    } else {
        println!("No employees found!")
    };
}


fn add_comp(comp: &mut HashMap<Department, Vec<Employee>>, empl: Employee, dept: Department) {
    comp.entry(dept).or_insert(Vec::new()).push(empl);
}   


fn main() {
    
    let mut comp: HashMap<Department, Vec<Employee>> = HashMap::new();

    loop {

        println!(
"Choose one operation:
1) Add employee
2) List department employees
3) List all employees
4) Quit programm"
        );

        let operation: Command = match read_line()
            .trim()
            .parse() {
                Ok(num) => if num >=1 && num <= 4 {
                    operations(num)    
                } else {
                    eprintln!("Error: Please insert a number from 1 to 4!\n");
                    continue;
                }
                Err(_)  => {
                    eprintln!("Error: Please insert a number from 1 to 4!\n");
                    continue;
                },
            };

        match operation {
            Command::Quit   => std::process::exit(0),
            Command::All    => list(&comp),
            Command::List{ department } => list_dept(&comp, &department),
            Command::Add{ employee, department } => add_comp(&mut comp, employee, department)
        }   
    };
}

