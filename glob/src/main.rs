static GLOBAL_VALUE: i32 = 12345;
const KONSTANTE: i32 = 54321;
const BOWLING: bool = true;

fn main() {
    println!("{GLOBAL_VALUE}");
    println!("{}", BOWLING);
    {
        const BOWLING: bool = false;
        println!("{}", BOWLING);
    }
    konstant();
    println!("{}", BOWLING);
    convert_type();
}

fn konstant() {
    // innerhalb eines Anweisungsblocks können konstanten neu zugewiesen werden
    // nach dem Block verlieren sie ihre Gültigkeit und nehmen wieder den globalen
    // Wert an.
    println!("{}", KONSTANTE);
    {
        const KONSTANTE: i32 = 4;
        println!("{}", KONSTANTE);
    }
    println!("{}", KONSTANTE);
}

fn convert_type() {
    let ganzzahl: i32 = 4_;

    let fp = ganzzahl as f64;
    let c = true as u32;

    println!("{},{},{}", ganzzahl, fp, c);
}

