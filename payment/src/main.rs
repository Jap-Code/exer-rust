#[derive(Debug, Clone)]
enum PaymentMethod {
    Cash,
    Voucher { promo_code: String, credit: f64 },
    CreditCard{ provider: String, number: String }
}

impl PaymentMethod {
    fn print_details(&self) {
        match self {
            PaymentMethod::Cash => println!("No further requirements! Just pay in cash when you are fetching your order."),
            PaymentMethod::Voucher{ promo_code , credit } => println!("Promo code: {:?}, amount left: {:?}$", promo_code, credit),
            PaymentMethod::CreditCard{ provider, number,} => println!("Card provider: {:?}, card number: {:?}", provider, number),
        }
    }
}

impl PaymentMethod {
    fn is_digital(&self) -> bool {
        match self {
            PaymentMethod::Cash => false,
            PaymentMethod::Voucher{ .. } | PaymentMethod::CreditCard{ .. } => true,
        }
    }
}

fn main() {

    let payment_methods: Vec<PaymentMethod> = vec![
        PaymentMethod::Cash,
        PaymentMethod::Voucher{ promo_code: String::from("Code123"), credit: 99.99 },
        PaymentMethod::CreditCard{ provider: String::from("Visa"), number: String::from("1234 2345 3456 4567")}
    ];

    for method in &payment_methods {
        method.print_details();
        if method.is_digital() {
            println!("Please confirm payment at the terminal!");
        }
    };

    println!("{:?}", payment_methods[0]);

    // hier wird nur ein vector mit referenzen auf den pyamentMethod vector erstellt (ein vector mit zeigern)
    // ist zwar performant, der referenzierte Verctor muss jedoch länger leben als die referenz und beide 
    // Referenz und Vector dürfen in dieser zeit nicht verändert werden.  
    let digital_payments: Vec<&PaymentMethod> = payment_methods
        .iter()
        .filter(|method| method.is_digital())
        .collect();

    println!("{:?}", digital_payments); 

    // hier erzeugen wir einen Clone mit den Items des Vectors PaymentMethods. Damit können wir letztendlich alls 
    // anstellen, unabhängig vom ursprügnlichen Vector und dessen Lebensdauer etc. Geht allerdings auf Kosten der 
    // Performance, da neuer Speicher angefordert und belegt werden muss. Erfordert den Trait #[derive(Clone)] im
    // Enum PaymentMethod.
    let digital_payments_clone: Vec<PaymentMethod> = payment_methods
        .iter()
        .filter(|method| method.is_digital())
        .cloned()
        .collect();

    println!("{:?}", digital_payments_clone); 
}

    // let user1 = PaymentMethod::Cash;
    // user1.print_details();
    // println!("{}", user1.is_digital());
    
    // let mut user2 = PaymentMethod::Voucher{ promo_code: String::from("CODE123"), credit: 34.99 };
    // user2.print_details();
    // user2 = PaymentMethod::Voucher{ promo_code: String::from("CODE123"), credit: 30.99 };
    // user2.print_details();
    // let user3 = PaymentMethod::CreditCard{ provider: String::from("Mastercard"), number: String::from("1234 4567 7890 3579")};
    // user3.print_details();
    // user3.is_digital();