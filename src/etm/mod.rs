mod addr;
pub mod controller;
pub mod find;
pub mod mode;
mod reg;
pub use controller::*;
pub use mode::EtmMode;

mod etmerror {
    pub enum ETMError {
        InvalidHex(String),
        InvalidAddrIdx(u8),
        InvalidAddrRange(String),
        AddrRangeLimitExceed,
    }

    impl std::fmt::Display for ETMError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ETMError::InvalidHex(hex) => {
                    write!(f, "Invalid hex string {}", hex)
                }
                ETMError::InvalidAddrIdx(idx) => {
                    write!(f, "Invalid Address Index {}", idx)
                }
                ETMError::InvalidAddrRange(addr) => {
                    write!(f, "Invalid Address Range {}", addr)
                }
                ETMError::AddrRangeLimitExceed => {
                    write!(f, "Address Range Limit Exceed")
                }
            }
        }
    }

    impl std::fmt::Debug for ETMError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ETMError::InvalidHex(hex) => {
                    write!(f, "Invalid hex string {}", hex)
                }
                ETMError::InvalidAddrIdx(idx) => {
                    write!(f, "Invalid Address Index {}", idx)
                }
                ETMError::InvalidAddrRange(addr) => {
                    write!(f, "Invalid Address Range {}", addr)
                }
                ETMError::AddrRangeLimitExceed => {
                    write!(f, "Address Range Limit Exceed")
                }
            }
        }
    }

    impl std::error::Error for ETMError {
        fn description(&self) -> &str {
            match self {
                ETMError::InvalidHex(_) => "Invalid hex string",
                ETMError::InvalidAddrIdx(_) => "Invalid Address Index",
                ETMError::InvalidAddrRange(_) => "Invalid Address Range",
                ETMError::AddrRangeLimitExceed => "Address Range Limit Exceed",
            }
        }
    }
}
