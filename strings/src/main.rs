fn main() {
    let empty = String::new();

    let some = "some".to_string();
    let same = String::from("same");

    // alles was utf-8 encoded ist funktioniert:
    let mut also_works = String::from("안녕하세요");
    println!("{also_works}");
    
    also_works.push_str(" 감사해요");
    println!("{also_works}");
    
    let s1 = String::from("lick");
    let s2 = String::from(" my butt");
    let s3 = s1 + &s2;

    // s1 ist nicht mehr gültig: fn add(self, s: &str) -> String
    // s2 schon, weil wir nur ne referenz addieren und add nicht ownership von s2 übernimmt. 
    dbg!(&s2, &s3);

    let n1 = String::from("tic");
    let n2 = String::from("tag");
    let n3 = String::from("toe");

    let s = format!("{}-{}-{}", n1, n2, n3);
    dbg!(&s);

    //format! übernimmt keine ownership, dementsprechend bleiben die n's gültig und verwendbar. 
    println!("{} {} {}", n1, n2, n3);

    // da in rust strings als utf-8 gespeichert werden und auch so behandelt werden, können einzelne
    // "Buchstaben" nicht per index abgerufen werden. Slices sind aus eben diesem grund auch schwierig
    // iteriert werden kann per .bytes() oder .chars(). 
}
