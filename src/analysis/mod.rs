use crate::data::ESTransaction;
use ethabi::ParamType;

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

#[derive(Debug)]
pub enum Withdraw<'a, T: ESTransaction> {
    WithRelayer(&'a T, &'a T),
    WithoutRelayer(&'a T),
}

const WITHDRAW_SIGNATURE: [ParamType; 7] = [
    ParamType::Bytes,
    ParamType::FixedBytes(32),
    ParamType::FixedBytes(32),
    ParamType::Address,
    ParamType::Address,
    ParamType::Uint(256),
    ParamType::Uint(256),
];

const DEPOSIT_SIGNATURE: [ParamType; 1] = [ParamType::FixedBytes(32)];

pub mod analyze;
pub mod prepare;
