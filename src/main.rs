use dialoguer::Select;
use reqwest::Error;

mod types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let items: Vec<&str> = vec!["USD", "BRL", "EUR"];

    let selection: usize = Select::new()
        .with_prompt("Currency input:")
        .items(&items)
        .interact()
        .unwrap();

    let selected_item: &str = items[selection];

    println!("You chose: {}", selected_item);
    let request_url: String = format!("https://open.er-api.com/v6/latest/{selected_item}");
    println!("{}", request_url);
    let response: reqwest::Response = reqwest::get(&request_url).await?;

    let exchange_rates: types::CurrencyExchange = response.json().await?;
    println!("{:?}", exchange_rates);
    Ok(())
}
