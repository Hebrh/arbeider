//! indicator data types.

use chrono::{NaiveDate, Utc};

/// Portfolio is a set of stocks or other financial assets.
/// One stock also can be a portfolio.
struct Portfolio {
    name: String,
    positions: Vec<Position>,
}

/// Position is amount of asset that you own.
struct Position {
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
struct Asset {
    /// The name of the asset.
    name: String,
    /// The code of the asset.like 600000.SH
    code: String,
    /// The kind of the asset.like stock, bond, etc.
    kind: AssetKind,
}

/// Asset price is a series of price of asset in a period.
struct Price {
    /// The asset of the price.
    asset: Asset,
    /// The price of the asset.
    price: Vec<f64>,
    /// The date of the price.
    date: Vec<NaiveDate>,
}


/// AssetKind is the type of asset.
enum AssetKind {
    Stock,
    Bond,
    Cash,
}