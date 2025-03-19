// imports
use chrono::{Duration, Utc};
use std::error::Error;
use time::OffsetDateTime;
use yahoo_finance_api as yahoo;

/// Fetch SEC filings for a given CIK and form type
/// 
/// Example URL: 
///     https://www.sec.gov/Archives/edgar/data/320193/000032019320000096/0000320193-20-000096.txt
/// 
/// The URL format is: 
///     https://www.sec.gov/Archives/edgar/data/{CIK}/{AccessionNumber-no-dashes}/{AccessionNumber-with-dashes}.txt
/// 
/// Returns: a vector of URLs for the filings
pub async fn fetch_sec_filings(cik: &str) -> Result<serde_json::Value, Box<dyn Error>> {
    let url = format!("https://data.sec.gov/api/xbrl/companyfacts/CIK{}.json", cik);

    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header("User-Agent", "optimizeme/1.0 (jleechris06@gmail.com)")
        .send()
        .await?
        .text()
        .await?;
    let json: serde_json::Value = serde_json::from_str(&response)?;
    Ok(json)
}

/// Get the CIK (Central Index Key) for a given stock ticker
/// 
/// This function reads a local JSON file containing mappings of stock tickers to CIKs.
/// The JSON file is based on the SEC's EDGAR company listings.
/// 
/// Returns: the CIK as a 10-digit string
pub fn get_cik(ticker: &str) -> Result<String, Box<dyn Error>> {
    // Read the embedded JSON file
    let json_data = include_str!("company_tickers.json");

    println!("Reading CIK data from local file");

    // Parse JSON
    let json: serde_json::Value = serde_json::from_str(json_data)?;

    // Iterate over all entries in the JSON object
    if let Some(obj) = json.as_object() {
        for (_, company) in obj {
            // Check if this entry matches our ticker
            if let Some(ticker_value) = company.get("ticker") {
                if ticker_value.as_str().unwrap_or("").to_uppercase() == ticker.to_uppercase() {
                    // Extract CIK (key is "cikstr" in the JSON)
                    if let Some(cik_value) = company.get("cik_str") {
                        if let Some(cik_num) = cik_value.as_u64() {
                            // Format CIK to 10 digits with leading zeros
                            return Ok(format!("{:010}", cik_num));
                        }
                    }
                }
            }
        }
    }

    Err(format!("CIK not found for ticker: {}", ticker).into())
}

/// Get historical stock price data for a given ticker
/// 
/// This function uses the Yahoo Finance API to fetch historical stock price data.
/// It retrieves the stock price data for the past `days` days.
/// 
/// Returns: a `YResponse` struct containing the historical price data
#[allow(unused)]
async fn get_stock_history(ticker: &str, days: i64) -> Result<yahoo::YResponse, Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new()?;

    let end = Utc::now();
    let start = end - Duration::days(days);

    println!("Fetching {} days of history for {}", days, ticker);

    let start_odt = OffsetDateTime::from_unix_timestamp(start.timestamp())?;
    let end_odt = OffsetDateTime::from_unix_timestamp(end.timestamp())?;

    let response = provider
        .get_quote_history(ticker, start_odt, end_odt)
        .await?;

    // Ensure we have valid data
    if response.quotes().is_err() {
        return Err("Failed to get quotes from Yahoo response".into());
    }

    Ok(response)
}

/// Get the latest stock quote for a given ticker
/// 
/// This function uses the Yahoo Finance API to fetch the latest stock quote for a given ticker.
/// 
/// Returns: a `YResponse` struct containing the latest quote data
#[allow(unused)]
async fn get_latest_quote(ticker: &str) -> Result<yahoo::YResponse, Box<dyn Error>> {
    let provider = yahoo::YahooConnector::new()?;

    println!("Fetching latest quote for {}", ticker);

    // Get the latest quotes
    let response = provider.get_latest_quotes(ticker, "1d").await?;

    Ok(response)
}

pub fn parse_json(json: &serde_json::Value) -> Vec<u64> {
    // extract revenue data from past 5 years to calculate growth rate
    let mut last_year = 0;
    let mut second_last_year = 0;
    let mut revenue_data = Vec::new();
    if let Some(data) = json.get("facts")
        .and_then(|f| f.get("us-gaap"))
        .and_then(|g| g.get("Revenues"))
        .and_then(|r| r.get("units"))
        .and_then(|u| u.get("USD")) {
        //TODO
    }
    
    revenue_data
}