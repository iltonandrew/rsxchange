use dialoguer::Select;
use reqwest::Error;
use strum::IntoEnumIterator;
mod types;

use field_accessor::FieldAccessor;
use serde::Deserialize;
use serde_derive::Serialize;
use strum::{Display, EnumDiscriminants, EnumIter};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FieldAccessor)]
pub struct CurrencyExchange {
    pub result: String,
    pub rates: Rates,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, FieldAccessor, PartialOrd)]
#[serde(rename_all = "UPPERCASE")]
pub struct Rates {
    pub usd: f64,
    pub brl: f64,
    pub gbp: f64,
    pub eur: f64,
}

#[derive(Debug, EnumIter, Display, EnumDiscriminants)]
pub enum Currencies {
    USD,
    BRL,
    EUR,
    GBP,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let items: Vec<Currencies> = Currencies::iter().collect::<Vec<_>>();

    let select_from_currency: usize = Select::new()
        .with_prompt("Currency input:")
        .items(&items)
        .interact()
        .unwrap();

    let selected_from_currency: &String = &items[select_from_currency].to_string().to_lowercase();

    let select_to_currency: usize = Select::new()
        .with_prompt("Currency input:")
        .items(&items)
        .interact()
        .unwrap();

    let selected_to_currency: &String = &items[select_to_currency].to_string().to_lowercase();

    let amount: f64 = dialoguer::Input::<f64>::new()
        .with_prompt("Amount to convert:")
        .interact()
        .unwrap();

    let request_url: String = format!("https://open.er-api.com/v6/latest/{selected_from_currency}");
    println!("{}", request_url);
    let response: reqwest::Response = reqwest::get(&request_url).await?;

    let exchange_rates: CurrencyExchange = response.json().await?;

    let currency_exchange: f64 = exchange_rates.rates.get(selected_to_currency).unwrap() * amount;
    println!(
        "{} {} = {} {}",
        amount, selected_from_currency, currency_exchange, selected_to_currency
    );
    Ok(())
}
