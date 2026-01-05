use rand::Rng;

fn main() {
    let months: [&str; 12]  = [
        "january",
        "febuary",
        "march",
        "april",
        "may",
        "june",
        "july",
        "august",
        "september",
        "october",
        "november",
        "december",
    ];

    for _ in months {
        let rng = rand::rng().random_range(0..=11);
        let response = match months[rng] {
            "january"   => "winter",
            "febuary"   => "winter",
            "march"     => "spring",
            "april"     => "spring",
            "may"       => "spring",
            "june"      => "summer",
            "july"      => "summer",
            "august"    => "summer",
            "september" => "autumn",
            "october"   => "autumn",
            "november"  => "autumn",
            "december"  => continue,
            _           => break,
        };
        println!("{} is a {} month", months[rng], response);
    }
}