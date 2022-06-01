mod implementations;

use hex_literal::hex;
pub mod analyze;
pub mod prepare;

const TORNADO_CASH_ROUTER: [u8; 20] = hex!("d90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b");

#[derive(PartialEq, Debug, Copy, Clone)]
#[allow(non_snake_case)]
pub struct DepositWithdrawPattern {
    pub n0_1ETH: u64,
    pub n1ETH: u64,
    pub n10ETH: u64,
    pub n100ETH: u64,
}
