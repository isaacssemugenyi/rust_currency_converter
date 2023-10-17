use std::io::stdin;

// A Struct to act as a return type for display_data() function
struct FinalResult {
    original_currency: String,
    original_amount: f32,
    final_currency: String,
    final_amount: f32,
}

// Main function that calles the display_data function to run the application
fn main() {
    let mut continue_conversion: String = String::new();

    let mut result = display_data();

    println!(
        "{:?} {:?} was converted to {:?} {:?}",
        result.original_currency,
        result.original_amount,
        result.final_currency,
        result.final_amount
    );


    // Loops for even when Y or Yes is typed in to perform another conversion
    loop {
        println!("Do you still want to perform another conversion");
        stdin()
            .read_line(&mut continue_conversion)
            .expect("Not a valid string");

        if continue_conversion.trim().to_lowercase() == "y" || continue_conversion.trim().to_lowercase() == "yes" {
            result = display_data();

            println!(
                "{:?} {:?} was converted to {:?} {:?}",
                result.original_currency,
                result.original_amount,
                result.final_currency,
                result.final_amount
            );

            // Reset the variable continue_conversion as it was concatinating values of the previous input
            continue_conversion = String::new(); 
        } else {
            println!("Thanks for using our currency converter. Come again next time");
            break
        }
    }
}

// Function pick all the neccessary user input for the conversion
fn display_data() -> FinalResult {
    let num: f32;
    let mut amount_input: String = String::new();
    let mut from_currency: String = String::new();
    let mut to_currency: String = String::new();

    println!("Enter a number:");
        stdin()
            .read_line(&mut amount_input)
            .expect("Not a valid string");
        num = amount_input
            .trim()
            .parse::<f32>()
            .expect("Not a valid number");

        println!("Enter currency type of amount entered:");

        stdin()
            .read_line(&mut from_currency)
            .expect("Not a valid string");

        println!("Enter currency type to convert to:");
        stdin()
            .read_line(&mut to_currency)
            .expect("Not a valid string");

        let result = match_from_currency(
            &from_currency.trim().to_uppercase(),
            &to_currency.trim().to_uppercase(),
            &num);


        return FinalResult {
            original_currency: String::from(&from_currency.trim().to_uppercase()),
            original_amount:  num,
            final_currency: String::from(&to_currency.trim().to_uppercase()),
            final_amount: result
        };
}

// match_from_currency matches which function to forward the conversion too incase a given curreny type from_currency is used
fn match_from_currency(from_currency: &str, to_currency: &str, num: &f32) -> f32 {
    match &*from_currency.trim() {
        "USD" => convert_from_usd(&to_currency, &num),
        "EUR" => convert_from_eur(&to_currency, &num),
        "GBP" => convert_from_gbp(&to_currency, &num),
        "KES" => convert_from_kes(&to_currency, &num),
        "UGX" => convert_from_ugx(&to_currency, &num),
        _ => 0.0,
    }
}

// Converts currency from UGX to any other -> USD, EUR, GBP, KES
// The same for the below functions
fn convert_from_ugx(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_ugx_rate = 4564.81;
    let eur_to_ugx_rate = 3955.52;
    let kes_to_ugx_rate = 25.168;
    let usd_to_ugx_rate = 3741.22;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_ugx_rate,
        "EUR" => *num * eur_to_ugx_rate,
        "GBP" => *num * gbp_to_ugx_rate,
        "KES" => *num * kes_to_ugx_rate,
        "UGX" => *num,
        _ => 0.0,
    };

    return amount;
}

fn convert_from_usd(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_usd_rate = 1.22014;
    let eur_to_usd_rate = 1.05728;
    let ugx_to_usd_rate = 0.00026;
    let kes_to_usd_rate = 0.00673;

    let amount: f32 = match to_currency {
        "EUR" => *num * eur_to_usd_rate,
        "GBP" => *num * gbp_to_usd_rate,
        "KES" => *num * kes_to_usd_rate,
        "UGX" => *num * ugx_to_usd_rate,
        "USD" => *num,
        _ => 0.0,
    };

    return amount;
}

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c81549461fa67e79df562daeafa5de63

fn convert_from_eur(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_eur_rate = 1.15335;
    let usd_to_eur_rate = 0.94554;
    let ugx_to_eur_rate = 0.00025;
    let kes_to_eur_rate = 0.00636;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_eur_rate,
        "GBP" => *num * gbp_to_eur_rate,
        "KES" => *num * kes_to_eur_rate,
        "UGX" => *num * ugx_to_eur_rate,
        "EUR" => *num,
        _ => 0.0,
    };

    return amount;
}

fn convert_from_gbp(to_currency: &str, num: &f32) -> f32 {
    let ugx_to_gbp_rate = 0.00022;
    let eur_to_gbp_rate = 0.86615;
    let kes_to_gbp_rate = 0.00551;
    let usd_to_gbp_rate = 0.81921;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_gbp_rate,
        "EUR" => *num * eur_to_gbp_rate,
        "KES" => *num * kes_to_gbp_rate,
        "UGX" => *num * ugx_to_gbp_rate,
        "GBP" => *num,
        _ => 0.0,
    };

    return amount;
}

fn convert_from_kes(to_currency: &str, num: &f32) -> f32 {
    let gbp_to_kes_rate = 180.275;
    let eur_to_kes_rate = 156.213;
    let ugx_to_kes_rate = 0.03905;
    let usd_to_kes_rate = 147.75;

    let amount: f32 = match to_currency {
        "USD" => *num * usd_to_kes_rate,
        "EUR" => *num * eur_to_kes_rate,
        "GBP" => *num * gbp_to_kes_rate,
        "UGX" => *num * ugx_to_kes_rate,
        "KES" => *num,
        _ => 0.0,
    };

    return amount;
}
