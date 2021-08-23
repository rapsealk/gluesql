use {
    super::Expr,
    serde::{Deserialize, Serialize},
    strum_macros::Display,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, Display)]
pub enum Function {
    #[strum(to_string = "LOWER")]
    Lower(Expr),
    #[strum(to_string = "UPPER")]
    Upper(Expr),
    #[strum(to_string = "LEFT")]
    Left { expr: Expr, size: Expr },
    #[strum(to_string = "RIGHT")]
    Right { expr: Expr, size: Expr },
    #[strum(to_string = "CEIL")]
    Ceil(Expr),
    #[strum(to_string = "ROUND")]
    Round(Expr),
    #[strum(to_string = "FLOOR")]
    Floor(Expr),
    #[strum(to_string = "TRIM")]
    Trim(Expr),
    #[strum(to_string = "DIV")]
    Div { dividend: Expr, divisor: Expr },
    #[strum(to_string = "MOD")]
    Mod { dividend: Expr, divisor: Expr },
    #[strum(to_string = "GCD")]
    Gcd { left: Expr, right: Expr },
    #[strum(to_string = "LCM")]
    Lcm { left: Expr, right: Expr },
    #[strum(to_string = "SIN")]
    Sin(Expr),
    #[strum(to_string = "COS")]
    Cos(Expr),
    #[strum(to_string = "TAN")]
    Tan(Expr),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Aggregate {
    Count(Expr),
    Sum(Expr),
    Max(Expr),
    Min(Expr),
}