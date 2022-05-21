use super::analysis::Withdraw;
use super::data::ESTransaction;

pub fn check_sum_per_deposit<T: ESTransaction>(deposits: &Vec<&T>) -> bool {
    deposits.iter().all(|t| {
        t.transaction_value() == 1_000_000_000_000_000_000 || t.transaction_is_error() != 0
    })
}

pub fn check_sum_per_withdraw<T: ESTransaction>(withdraws: &Vec<Withdraw<T>>) -> bool {
    withdraws.iter().all(|t| match t {
        Withdraw::WithoutRelayer(t) => {
            t.transaction_value() == 1_000_000_000_000_000_000 || t.transaction_is_error() != 0
        }
        Withdraw::WithRelayer(t, f) => {
            t.transaction_value() + f.transaction_value() == 1_000_000_000_000_000_000
                || t.transaction_is_error() != 0
        }
    })
}
