//! indicator data types.

use chrono::NaiveDate;

/// Portfolio is a set of stocks or other financial assets.
/// One stock also can be a portfolio.
#[allow(dead_code)]
pub struct Portfolio {
    name: String,
    positions: Vec<Position>,
}

/// Position is amount of asset that you own.
#[allow(dead_code)]
pub struct Position {
    /// The asset of the position.
    asset: Asset,
    /// The quantity of the position.
    quantity: f64,
    /// Buy date of the position.
    buy_date: NaiveDate,
    /// Sell date of the position.
    sell_date: NaiveDate,
}

/// Asset include stock, bond, etc.
#[allow(dead_code)]
pub struct Asset {
    /// The name of the asset.
    name: String,
    /// The code of the asset.like 600000.SH
    code: String,
    /// The kind of the asset.like stock, bond, etc.
    kind: AssetKind,
}

/// Asset price is a series of price of asset in a period.
#[allow(dead_code)]
pub struct Price {
    /// The asset of the price.
    asset: Asset,
    /// The price of the asset.
    price: Vec<f64>,
    /// The date of the price.
    date: Vec<NaiveDate>,
}

// Datetime period.
#[derive(Debug, Clone, Copy)]
pub struct Period {
    pub start: NaiveDate,
    pub end: NaiveDate,
}

/// AssetKind is the type of asset.
pub enum AssetKind {
    Stock,
    Bond,
    Cash,
}

/// Category of stock
pub enum StockCategory {
    /// Shanghai Stock Exchange，0
    SH,
    /// Shenzhen Stock Exchange, 1
    SZ,
    /// Startup Edition stock exchange. 创业版, 2
    CY,
    /// Tech board stock exchange. 科创板, 3
    KB,
}