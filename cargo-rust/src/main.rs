#[derive(Debug)]
enum SeatCategory {
    Standard, 
    Premium { surcharge: f64 }, 
    VIP { surcharge: f64, snack_packet: SnackPacket },
}

#[derive(Debug)]
enum SnackPacket {
    Popcorn,
    Nachos,
}

#[derive(Debug)]
enum BookingError {
    InsufficentBudget,
    InvalidRequest,
}

#[derive(Debug)]
struct TicketRequest {
    id: u32,
    base_price: f64,
    category: SeatCategory,
}


impl TicketRequest {
    fn calculate_total(&self) -> (f64, Option<&SnackPacket>){

        let (surcharge, snack) = match &self.category {
            SeatCategory::Standard                      => (0.0, None),
            SeatCategory::Premium{ surcharge }          => (*surcharge, None),
            SeatCategory::VIP{ surcharge, snack_packet} => (*surcharge, Some(snack_packet)),
        };
        ((self.base_price + surcharge), snack)
    }
}

fn main() {

    let mut budget = 50.00;

    let reqs: Vec<Option<TicketRequest>> = vec![
        Some(TicketRequest{ id: 1, base_price: 12.50, category: SeatCategory::VIP{ surcharge: 7.50, snack_packet: SnackPacket::Popcorn}}),
        Some(TicketRequest{ id: 2, base_price: 52.50, category: SeatCategory::Standard}),
        Some(TicketRequest{ id: 3, base_price: 12.50, category: SeatCategory::Premium {surcharge: 5.0}}),
        None,
    ];

    let rejection_log: Vec<(u32, BookingError)> = reqs
        .iter()
        .filter_map(|x| match x {
            None    => Some((0, BookingError::InvalidRequest)),
            Some(t) => {
                match t.calculate_total() {
                    val if val.0 > budget => Some((t.id, BookingError::InsufficentBudget)),
                    val                   => {
                        budget -= val.0;
                        None
                    },
                }
            },
        })
        .collect();

    dbg!(&rejection_log);
    dbg!(&budget);

}