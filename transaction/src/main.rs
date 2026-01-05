#[derive(Debug, Clone)]
enum TransactionType {
    Deposit,
    Withdrawal,
}

#[derive(Debug, Clone)]
enum TransactionError {
    InsufficentFunds,
    AmountTooLow,
    InvalidData,
}

#[derive(Debug, Clone)]
struct Transaction {
    id: u32,
    amount: f64,
    t_type: TransactionType,
}

impl Transaction {
    fn process(&self, current_balance: f64) -> Result<f64, TransactionError> {
        
        match self {
            Transaction{ amount, .. } if *amount < 1.0 => {
                Err(TransactionError::AmountTooLow)
            }
            Transaction{ t_type: TransactionType::Withdrawal, amount, .. } if *amount > current_balance => {
                Err(TransactionError::InsufficentFunds)
            },
            Transaction{ t_type: TransactionType::Deposit, amount, .. } => {
                Ok(current_balance + *amount )
            },
            Transaction{ t_type: TransactionType::Withdrawal, amount, .. } => {
                Ok(current_balance - *amount )
            },
        }
    
    }
}

fn main() {
    let mut current_balance: f64 = 100.00;

    let transactions: Vec<Option<Transaction>> = vec![
        None,
        Some(Transaction {
            id: 1,
            amount: 35.99,
            t_type: TransactionType::Withdrawal,
        }),
        Some(Transaction {
            id: 2,
            amount: 12.00,
            t_type: TransactionType::Deposit,
        }),
        Some(Transaction {
            id: 3,
            amount: 102.95,
            t_type: TransactionType::Withdrawal,
        }),
        Some(Transaction {
            id: 4,
            amount: 0.73,
            t_type: TransactionType::Deposit,
        }),
        Some(Transaction {
            id: 5,
            amount: 76.00999999999999,
            t_type: TransactionType::Withdrawal,
        })

    ];

    let error_log: Vec<(u32, TransactionError)> = transactions
        .iter()
        .filter_map(|t| {
            match t {
                None => Some((0, TransactionError::InvalidData)),
                Some(trans) => {
                    match trans.process(current_balance) {
                        Ok(value) => {
                            current_balance = value;
                            None
                        },
                        Err(err) => Some((trans.id, err))
                    }
                }
            }
        })
        .collect();

    println!("{:#?}", error_log);
    dbg!(&current_balance);

}