use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ExchangeName {
    UniswapV2,
    SushiSwapV2,
    UniswapV3,
    SushiSwapV3,
    Unknown,
}

impl fmt::Display for ExchangeName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ExchangeName {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "uniswapv2" => Ok(ExchangeName::UniswapV2),
            "sushiswapv2" => Ok(ExchangeName::SushiSwapV2),
            "uniswapv3" => Ok(ExchangeName::UniswapV3),
            "sushiswapv3" => Ok(ExchangeName::SushiSwapV3),
            "unknown" => Ok(ExchangeName::Unknown),
            _ => Err(format!("Invalid exchange name: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ExchangeName::UniswapV2 => "uniswapv2",
            ExchangeName::SushiSwapV2 => "sushiswapv2",
            ExchangeName::UniswapV3 => "uniswapv3",
            ExchangeName::SushiSwapV3 => "sushiswapv3",
            ExchangeName::Unknown => "unknown",
        }
    }
}

impl Default for ExchangeName {
    fn default() -> Self {
        ExchangeName::Unknown
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum ExchangeType {
    UniV2,
    UniV3,
    Unknown,
}

impl ExchangeType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "univ2" => Ok(ExchangeType::UniV2),
            "univ3" => Ok(ExchangeType::UniV3),
            _ => Err(format!("Invalid exchange type: {}", s)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ExchangeType::UniV2 => "univ2",
            ExchangeType::UniV3 => "univ3",
            ExchangeType::Unknown => "unknown",
        }
    }
}

impl Default for ExchangeType {
    fn default() -> Self {
        ExchangeType::Unknown
    }
}
