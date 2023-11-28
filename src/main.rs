use dialoguer::Select;
use reqwest::Error;
use strum::{EnumIter, IntoEnumIterator};

mod types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let items = types::Currencies::iter().collect::<Vec<_>>();

    let selection: usize = Select::new()
        .with_prompt("Currency input:")
        .items(&items)
        .interact()
        .unwrap();

    let selected_item: &types::Currencies = &items[selection];
    let request_url: String = format!("https://open.er-api.com/v6/latest/{selected_item}");
    println!("{}", request_url);
    let response: reqwest::Response = reqwest::get(&request_url).await?;

    let exchange_rates: types::CurrencyExchange = response.json().await?;
    println!("{:?}", exchange_rates);
    Ok(())
}
