mod implementations;

use hex_literal::hex;

#[macro_export]
macro_rules! hashstring {
    ($l:expr) => {
        format!(
            "0x{}",
            $l.as_bytes()
                .iter()
                .map(|b| format!("{:02x}", b))
                .collect::<String>()
        )
    };
}

#[macro_export]
macro_rules! immut_append {
    ($l:expr, $a:expr) => {
        $l.into_iter().chain([$a].into_iter()).collect()
    };
}

pub mod analyze;
pub mod prepare;

const TORNADO_CASH_ROUTER: [u8; 20] = hex!("d90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b");

#[derive(PartialEq)]
struct DepositWithdrawPattern {
    pub n0_1ETH: u64,
    pub n1ETH: u64,
    pub n10ETH: u64,
    pub n100ETH: u64,
}
