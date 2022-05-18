use super::{Withdraw, DEPOSIT_SIGNATURE, WITHDRAW_SIGNATURE};
use crate::data::{ESNormalTransaction, ESTransaction};
use ethabi::short_signature;
use hex_literal::hex;
use std::collections::HashMap;
use web3::types::H256;

const TORNADO_CASH_1ETH: [u8; 20] = hex!("47ce0c6ed5b0ce3d3a51fdb1c52dc66a7c3c2936");

pub fn divide_to_from<'a, T: ESTransaction>(transactions: &'a Vec<T>) -> (Vec<&'a T>, Vec<&'a T>) {
    transactions.iter().fold((vec![], vec![]), |(to, from), t| {
        if t.transaction_to().is_some() && t.transaction_to().unwrap() == TORNADO_CASH_1ETH.into() {
            (to.into_iter().chain([t].into_iter()).collect(), from)
        } else {
            (to, from.into_iter().chain([t].into_iter()).collect())
        }
    })
}

pub fn divide_withdraw_deposit<'a>(
    transactions: &'a Vec<ESNormalTransaction>,
) -> (
    Vec<&'a ESNormalTransaction>,
    Vec<&'a ESNormalTransaction>,
    Vec<&'a ESNormalTransaction>,
) {
    let withdraw_sig: [u8; 4] = short_signature("withdraw", &WITHDRAW_SIGNATURE);
    let deposit_sig: [u8; 4] = short_signature("deposit", &DEPOSIT_SIGNATURE);

    transactions.iter().fold(
        (vec![], vec![], vec![]),
        |(deposits, withdraws, others), t| match &t.input {
            Some(i) => {
                let sig: [u8; 4] = if i.len() >= 4 {
                    i[0..4].try_into().unwrap()
                } else {
                    [0u8; 4]
                };
                if sig == deposit_sig {
                    (
                        deposits.into_iter().chain([t].into_iter()).collect(),
                        withdraws,
                        others,
                    )
                } else if sig == withdraw_sig {
                    (
                        deposits,
                        withdraws.into_iter().chain([t].into_iter()).collect(),
                        others,
                    )
                } else {
                    (
                        deposits,
                        withdraws,
                        others.into_iter().chain([t].into_iter()).collect(),
                    )
                }
            }
            None => (
                deposits,
                withdraws,
                others.into_iter().chain([t].into_iter()).collect(),
            ),
        },
    )
}

pub fn group_fee_withdraws<'a, T: ESTransaction + PartialEq>(
    sent: Vec<&'a T>,
) -> Result<Vec<Withdraw<'a, T>>, String> {
    let mut transaction_mapping: HashMap<H256, Vec<&T>> = sent
        .iter()
        .map(|t| (t.transaction_hash(), vec![]))
        .collect();

    sent.iter().for_each(|t| {
        transaction_mapping.insert(
            t.transaction_hash(),
            transaction_mapping
                .get(&t.transaction_hash())
                .unwrap()
                .into_iter()
                .map(|s| *s)
                .chain([*t].into_iter())
                .collect(),
        );
    });

    let keys: Vec<&H256> = transaction_mapping.keys().collect();

    keys.into_iter().fold(Ok(vec![]), |r, h| {
        let parts = transaction_mapping
            .get(h)
            .unwrap()
            .iter()
            .fold(vec![], |v, p| {
                // sort out internal transactions that are included multiple times
                if v.contains(p) {
                    v
                } else {
                    v.into_iter().chain([*p].into_iter()).collect()
                }
            });

        match parts.len() {
            1 => Ok(r
                .unwrap()
                .into_iter()
                .chain([Withdraw::WithoutRelayer(parts[0])].into_iter())
                .collect()),
            2 => {
                let max = parts.iter().max_by_key(|t| t.transaction_value()).unwrap();
                let min = parts.iter().min_by_key(|t| t.transaction_value()).unwrap();

                Ok(r.unwrap()
                    .into_iter()
                    .chain(
                        // we assume that the fee is lower than the withdrawn amount
                        [Withdraw::WithRelayer(*min, *max)].into_iter(),
                    )
                    .collect())
            }
            _ => Err(format!(
                "Parsing error while parsing transaction {}: too many recipients {}",
                hashstring!(h),
                parts
                    .iter()
                    .map(|t| hashstring!(t.transaction_to().unwrap()))
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
        }
    })
}
