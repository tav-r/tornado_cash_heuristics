use super::DepositWithdrawPattern;

impl From<(u64, u64, u64, u64)> for DepositWithdrawPattern {
    #[allow(non_snake_case)]
    fn from((n0_1ETH, n1ETH, n10ETH, n100ETH): (u64, u64, u64, u64)) -> Self {
        Self {
            n0_1ETH,
            n1ETH,
            n10ETH,
            n100ETH,
        }
    }
}
