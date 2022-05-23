use super::DepositWithdrawPattern;

impl Into<DepositWithdrawPattern> for (u64, u64, u64, u64) {
    fn into(self) -> DepositWithdrawPattern {
        #[allow(non_snake_case)]
        let (n0_1ETH, n1ETH, n10ETH, n100ETH) = self;

        DepositWithdrawPattern {
            n0_1ETH,
            n1ETH,
            n10ETH,
            n100ETH,
        }
    }
}
