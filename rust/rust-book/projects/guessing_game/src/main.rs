use std::cmp::Ordering; //vergleich
use std::io;            //input

use rand::Rng;          //zufallsgenerator

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);  //generiere eine zufällige Zahl zwischen 1 und 100
    //println!("The secret number is {secret_number}");           //gebe diese Zahl in der Konsole aus

    loop {                              
        println!("Please input your guess.");                   //schleife für den input zum Zahlenraten

        let mut guess = String::new();                          //default => imuatable. Hier mutable variable geratene Zahl. Neuen leeren String zuweisen

        io::stdin()                                             //input vom stdin des io pakets (library)
            .read_line(&mut guess)                              //-> lies den input vom user, und weise ihn der mutable variable guess zu
            .expect("Failed to read line");                     //wenns nicht klappt.. 

        let guess: u32 = match guess.trim().parse() {           //guess verarbeiten: u32 zeichen, check ob die getrimmte und geparste version 
            Ok(num) => num,                                     //eine num ist, dann num
            Err(_) => continue,                                 //wenn nicht, dann fehler und es geht weiter mit neuem start der loop
        };
        
        println!("You guessed: {guess}");                       //feedback über userinput

        match guess.cmp(&secret_number) {                       //compare guess mit der secret number
            Ordering::Less => println!("Too small!"),           //ordering paket: kleiner... und von vorne
            Ordering::Greater => println!("Too big!"),          //ordering paket: größer... und von vorne
            Ordering::Equal => {                                //ordering paket: gleich.. 
                println!("You win!");                           //wenn gleich dann yuhuuu
                break;                                          //beende das ganze
            }
        }
    }
}