
//Enums in Rust 

enum Payment{
    Cash(f64),
    Credit(String,f64),
    Debit(debitData),
    Crypto{account_id: String, amount: f64},
}

struct debitData{
    account_number: String,
    amount: f64,
}

fn process_payment(some_payment: &Payment){
    
    match some_payment{
        Payment::Cash(amount) => {
            println!("Payment is cash in the amount of {}", amount);    
        }
        Payment::Credit(some_string, some_float) => {
            println!("Payment is credit and payment is id {} in the amount of {}", some_string, some_float);
        }
        Payment::Debit(data) => {
            println!("Payment is debit and payment is id {} in the amount of {}", data.account_number, data.amount);
        }
        _ => {
            println!("Payment is unknown");
        }
    }
}

fn main() {
    
    let some_payment = Payment::Cash(100.0);
    process_payment(&some_payment);

    let some_payment = Payment::Credit("1234567890".to_string(), 100.0);
    process_payment(&some_payment);

    let some_payment = Payment::Debit(debitData{
        
            account_number: "1234567890".to_string(),
             amount: 100.0
        });
    process_payment(&some_payment); 

    let some_payment = Payment::Crypto{account_id: "1234567890".to_string(), amount: 100.0};
    process_payment(&some_payment); 
    
}
