/*
 * BlockScout API
 *
 * API for BlockScout web app
 *
 * The version of the OpenAPI document: 1.0.0
 * Contact: you@your-company.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TokenInfoDetailed {
    /// Token Address
    #[serde(rename = "address")]
    pub address: String,
    /// Token circulating market cap
    #[serde(rename = "circulating_market_cap")]
    pub circulating_market_cap: String,
    /// Token decimals
    #[serde(rename = "decimals")]
    pub decimals: String,
    /// Token exchange rate
    #[serde(rename = "exchange_rate")]
    pub exchange_rate: String,
    /// Token holders amount
    #[serde(rename = "holders")]
    pub holders: String,
    /// Token image URL
    #[serde(rename = "icon_url")]
    pub icon_url: String,
    /// Token name
    #[serde(rename = "name")]
    pub name: String,
    /// Token symbol
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Token total supply
    #[serde(rename = "total_supply")]
    pub total_supply: String,
    /// Token type
    #[serde(rename = "type")]
    pub r#type: String,
    /// Token trading volume for past 24h
    #[serde(rename = "volume_24h")]
    pub volume_24h: String,
}

impl TokenInfoDetailed {
    pub fn new(
        address: String,
        circulating_market_cap: String,
        decimals: String,
        exchange_rate: String,
        holders: String,
        icon_url: String,
        name: String,
        symbol: String,
        total_supply: String,
        r#type: String,
        volume_24h: String,
    ) -> TokenInfoDetailed {
        TokenInfoDetailed {
            address,
            circulating_market_cap,
            decimals,
            exchange_rate,
            holders,
            icon_url,
            name,
            symbol,
            total_supply,
            r#type,
            volume_24h,
        }
    }
}
