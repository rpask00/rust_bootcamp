#![allow(dead_code)]


use std::{collections::HashMap, io, fmt, error::Error};
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use error_stack::{Report, ResultExt, Result};


#[derive(Debug)]
struct ParsePaymentInfoError;

impl Display for ParsePaymentInfoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Parsing payment error: invalid payment info")
    }
}

impl Error for ParsePaymentInfoError {}

fn parse_card_numbers(card: &str) -> Result<Vec<u32>, ParsePaymentInfoError> {
    let numbers = card
        .split(" ")
        .map(|s| {
            s.parse::<u32>()
                .attach_printable_lazy(|| {
                    format!("{s:?} could not be parsed as u32")
                })
        })
        .collect::<Result<Vec<u32>, ParseIntError>>()
        .change_context(ParsePaymentInfoError)
        .attach_printable(format!(
            "Failed to parse input as numbers. Input: {card}"
        ))?;

    Ok(numbers)
}

#[derive(Debug)]
struct Expiration {
    year: u32,
    month: u32,
}

#[derive(Debug)]
struct Card {
    number: u32,
    exp: Expiration,
    cvv: u32,
}

fn parse_card(card: &str) -> Result<Card, ParsePaymentInfoError> {
    let mut numbers = parse_card_numbers(card)?;

    let len = numbers.len();
    let expected_len = 4;

    if len != expected_len {
        return Err(Report::new(ParsePaymentInfoError).attach_printable(
            format!("Incorrect number of elements parsed. Expected {expected_len} but got {len}. Elements: {numbers:?}")
        ));
    }

    let cvv = numbers.pop().unwrap();
    let year = numbers.pop().unwrap();
    let month = numbers.pop().unwrap();
    let number = numbers.pop().unwrap();

    Ok(Card {
        number,
        exp: Expiration { year, month },
        cvv,
    })
}

#[derive(Debug)]
enum CreditCardError {
    InvalidInput(String),
    Other,
}

impl Display for CreditCardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CreditCardError::InvalidInput(msg) => f.write_str(msg),
            CreditCardError::Other => f.write_str("Something went wrong! Try again!"),
        }
    }
}

impl Error for CreditCardError {}

fn get_credit_card_info(
    credit_cards: &HashMap<&str, &str>,
    name: &str,
) -> Result<Card, CreditCardError> {
    let card_string = credit_cards.get(name).ok_or_else(|| {
        let msg = format!("No credit card was found for {name}.");
        Report::new(CreditCardError::InvalidInput(msg))
    })?;

    let card = parse_card(card_string)
        .change_context(CreditCardError::Other)
        .attach_printable(format!("{name}'s card could not be parsed."))?;

    Ok(card)
}

fn main() {
    env_logger::init();

    let credit_cards = HashMap::from([
        ("Amy", "1234567 12 16 123"),
        ("Tim", "1234567 0616 123"),
        ("Bob", "1234567 Dec 08 123"),
    ]);

    println!("Enter Name: ");

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read line");

    let result = get_credit_card_info(&credit_cards, name.trim());

    match result {
        Ok(card) => println!("\nCredit Card Info: {card:?}"),
        Err(err) => {
            match &err.current_context() {
                CreditCardError::InvalidInput(msg) => println!("{msg}"),
                CreditCardError::Other => println!("\nSomething went wrong! Try again!")
            }

            log::error!("\n{err:?}");
        }
    }
}
