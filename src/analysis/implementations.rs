use super::DepositWithdrawPattern;
use crate::data::{InPool, Pool};

impl<T: InPool> From<&Vec<&T>> for DepositWithdrawPattern {
    fn from(transactions: &Vec<&T>) -> Self {
        #[allow(non_snake_case)]
        let (n0_1ETH, n1ETH, n10ETH, n100ETH) = transactions.iter().fold(
            (0u64, 0u64, 0u64, 0u64),
            |(_0_1eth, _1eth, _10eth, _100eth), t| match t.pool() {
                Pool::_0_1ETH => (_0_1eth + 1, _1eth, _10eth, _100eth),
                Pool::_1ETH => (_0_1eth, _1eth + 1, _10eth, _100eth),
                Pool::_10ETH => (_0_1eth, _1eth, _10eth + 1, _100eth),
                Pool::_100ETH => (_0_1eth, _1eth, _10eth, _100eth + 1),
                Pool::Unknown => (_0_1eth, _1eth, _10eth, _100eth),
            },
        );

        DepositWithdrawPattern {
            n0_1ETH,
            n1ETH,
            n10ETH,
            n100ETH,
        }
    }
}
