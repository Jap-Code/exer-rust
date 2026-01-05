use rand::Rng;

#[derive(Debug)]
enum Packages {
    Standard { weight: f64 },
    Fragile { weight: f64, level: u32 },
    Express { weight: f64, priority: Priority },
}

impl Packages {
    fn calculate_shipping(&self) -> f64 {
        match self {
            Packages::Standard { weight }           => weight.ceil() * 1.0,
            Packages::Fragile { weight, .. }        => weight.ceil() * 1.5,
            Packages::Express { weight, priority }  => {
                let base = weight.ceil() * 2.0;
                let fee = match priority {
                    Priority::High      => 5.0,
                    Priority::Medium    => 2.0,
                    Priority::Low       => 0.0, 
                };
                base + fee
            },
        }
    }
}

#[derive(Debug)]
enum Priority {
    Low,
    Medium,
    High,
}

fn find_package_by_index(packages: &[Packages], index: usize) -> Option<&Packages> {
    // das hier lässt sich komplett auf eine zeile verkürzen, weil .get() schon genau das zurückgibt:
    // match packages.get(index) {
    //     Some(p) => Some(p),
    //     None    => None, 
    // }
    packages.get(index)
}

fn main() {
    // for testing purposes
    let p1 = Packages::Express{ weight: 2.5, priority: Priority::High };
    println!("Shipping costs: {} $", p1.calculate_shipping());
    
    let p2 = Packages::Express{ weight: 6.3, priority: Priority::Medium };
    println!("Shipping costs: {} $", p2.calculate_shipping());

    let p3 = Packages::Fragile{ weight: 1.0, level: 3 };
    println!("Shipping costs: {} $", p3.calculate_shipping());

    //actual assignment
    let packages: Vec<Packages> = vec![
        Packages::Standard{ weight: 6.8 },
        Packages::Standard{ weight: 3.1 },
        Packages::Fragile{ weight: 10.1, level: 5 },
        Packages::Fragile{ weight: 10.9, level: 3 },
        Packages::Express{ weight: 3.0, priority: Priority::High },
        Packages::Express{ weight: 2.5, priority: Priority:: Medium },
    ];

    let expensive_packages: Vec<&Packages> = packages
        .iter()
        .filter(|p| p.calculate_shipping() > 10.0)
        .collect();

    println!("{:?}", expensive_packages);

    if let Some(p) = find_package_by_index(&packages, rand::rng().random_range(0..=10)) {
        println!("Package found, shipping costs: {}$", p.calculate_shipping());
    } else {
        println!("No package found!");
    }
}