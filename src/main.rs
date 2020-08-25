use std::env;
use std::fs::File;
use std::io::prelude::*;

const APP_NAME: &str = "Forex Retirement Calculator";

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut years: u8 = 25; // Years to calculate
    let mut factor: f32 = 1.00001; // daily added value
    let mut capital: f32 = 250.00; // starting capital
    let mut monthly_payment: f32 = 250.00;
    let mut yearly_income: f32 = 27840.00; // net income
    let mut dividend: f32 = 0.1; // percentage of dividend from net profit
    let mut file_name: &str = "./money_money_money.txt"; // default output file name

    if args.len() == 8 {
        years = args[1].trim().parse::<u8>().unwrap();
        factor = args[2].trim().parse::<f32>().unwrap();
        capital = args[3].parse::<f32>().unwrap();
        monthly_payment = args[4].parse::<f32>().unwrap();
        yearly_income = args[5].trim().parse::<f32>().unwrap();
        dividend = args[6].trim().parse::<f32>().unwrap();
        file_name = args[7].trim();
    } else {
        println!("Usage of '{}' is as follows:", APP_NAME);
        println!("First argument: years to calculate (max. 256; e.g. {}),", years);
        println!("Second argument: daily factor (max. of f32 data type; e.g. {}),", factor);
        println!("Third argument: starting capital (max. of f32 data type; e.g. {:.2}),", capital);
        println!("Fourth argument: monthly payment/amount of saving (max. of f32 data type; e.g. {:.2}),", monthly_payment);
        println!("Fifth argument: net value of your yearly income (max. of f32 data type; e.g. {:.2}),", yearly_income);
        println!("Sixth argument: yearly dividend fromt net profit (max. of f32 data type; e.g. {}).", dividend);
        println!("Seventh argument: file_name (max. of string data type; e.g. {}).", file_name);
        println!("All above given examples are also the default values. You can not omit any argument.");
    }

    let mut year_df = 0.00;
    let mut taxes = 0.00;
    let mut payment_counter = 0;

    let f = File::create(&file_name).expect("couldn'tn't create file");

    writeln!(&f, "{}", format!("Starting capital is {:.2}€.", capital))
        .expect("couldn't write to file");

    for _counter in 1..years + 1 {
        for _month in 1..13 {
            let month_cap = capital;

            for _day in 1..31 {
                capital *= factor;
            }

            let month_dif = capital - month_cap;

            year_df += month_dif;

            writeln!(&f, "{}", format!("Monthly interest: {:.2}€.", month_dif))
                .expect("couldn't write to file");

            // don't push in monthly if the monthly difference is the accumulated yearly payment
            if month_dif <= yearly_income {
                capital += monthly_payment;
                writeln!(
                    &f,
                    "{}",
                    format!("Monthly payment of: {}€.", monthly_payment)
                )
                .expect("couldn't write to file");

                payment_counter += 1;
                writeln!(
                    &f,
                    "{}",
                    format!("Monthly payment number {}.", payment_counter)
                )
                .expect("couldn't write to file");
            }
        }

        writeln!(&f, "{}", format!("Yearly difference: {:.2}€.", year_df))
            .expect("couldn't write to file");

        // yearly capital taxes
        let mut taxes_per_year = 0.00;

        if year_df > 801.00 {
            taxes_per_year = (year_df - 801.00) * 0.3;
            taxes += taxes_per_year;

            writeln!(
                &f,
                "{}",
                format!("Yearly amount of capital tax paid: {:.2}€.", taxes_per_year)
            )
            .expect("couldn't write to file");

            // net profit + capital of last year when being taxed
            capital += year_df - taxes_per_year;
        }

        // capital after paying the dividend
        capital = capital - (year_df - taxes_per_year) * dividend;
        writeln!(
            &f,
            "{}",
            format!(
                "Payed {}% dividend: {:.2}€.",
                dividend * 100.00,
                (year_df - taxes_per_year) * dividend
            )
        )
        .expect("couldn't write to file");

        year_df = 0.00;
    }

    writeln!(
        &f,
        "{}",
        format!("{:.2}€ Kapital nach {} Jahren.", capital, years)
    )
    .expect("couldn't write to file");
    writeln!(
        &f,
        "{}",
        format!("{:.2}€ Steuern nach {} Jahren.", taxes, years)
    )
    .expect("couldn't write to file");
    println!("{}", format!("Ergebnis unter {}.", file_name));
}
