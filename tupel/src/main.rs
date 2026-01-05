// tupel werden mit einer oder mehreren Variablen 
// durch ein komma getrennt innerhalb runden Klammern 
// erzeugt. Auf die werte kann mit dem Index zugegriffen werden
// 
fn main(){
    let letter = 'r';
    let mut tupel = (letter, 2, 3.1);
    println!("{:?}", tupel);
    tupel.1 = 4;
    let (a, _, mut b) = tupel; //der zweite wert wird hier ausgelassen
    println!("{:#?}, {}", tupel, a);
    println!("{} {} {}", tupel.0, tupel.1, tupel.2);
}