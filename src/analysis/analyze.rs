use super::DepositWithdrawPattern;
use crate::data::{Deposit, Pool, Withdraw};
use std::collections::HashMap;
use web3::types::H160;

fn pattern_is_interesting(pattern: &DepositWithdrawPattern) -> bool {
    (pattern.n0_1ETH > 1 || pattern.n1ETH > 1 || pattern.n10ETH > 1 || pattern.n100ETH > 1)
        && [
            pattern.n0_1ETH,
            pattern.n1ETH,
            pattern.n10ETH,
            pattern.n100ETH,
        ]
        .into_iter()
        .filter(|e| *e > 0)
        .count()
            > 1
}

/// Returns a HashMap that assigns to each address that ever deposited ether a tuple
/// consisting of a vector of deposits by this address and a vector of withdraws by
/// this same address. If any of these vectors are empty, the address will not be
/// mapped.
///
/// Notice that this function does not check to which pools deposit/withdraw calls
/// were sent - the vectors must be filtered before.
///
/// # Arguments
///
/// * `deposits` - a vector of references to Deposit structures to scan
/// * `withdraws` - a vector of references to Withdraw structures to scan
pub fn get_address_matches<'a>(
    deposits: &'a Vec<&'a Deposit>,
    withdraws: &'a Vec<&'a Withdraw>,
) -> HashMap<H160, (Vec<&'a Deposit>, Vec<&'a Withdraw>)> {
    deposits
        .iter()
        // get all depositors without duplicates
        .fold(vec![], |addrs, d| {
            if !addrs.contains(&d.from) {
                immut_append!(addrs, d.from)
            } else {
                addrs
            }
        })
        .into_iter()
        // get all deposits and withdraws for each of the depositors
        .map(|a| {
            (
                a,
                (
                    deposits
                        .iter()
                        .filter(|d| d.from == a)
                        .map(|x| *x)
                        .collect(),
                    withdraws
                        .iter()
                        .filter(|w| w.receiver == a)
                        .map(|x| *x)
                        .collect(),
                ),
            )
        })
        // create tuples that pair each address with two vectors of deposits and withdraws after the
        // earliest deposits, yield only those for which there are any such withdraws
        .filter_map(|(a, (ds, ws)): (H160, (Vec<&Deposit>, Vec<&Withdraw>))| {
            // get all withdraws later than the earliest deposit
            let later_ws: Vec<&Withdraw> = ws
                .into_iter()
                .filter(|w| w.block_number > ds.iter().map(|d| d.block_number).min().unwrap_or(0))
                .collect();

            // check if any such withdraws were found, if yes, yield the match
            if ds.len() > 0 && later_ws.len() > 0 {
                Some((a, (ds, later_ws)))
            } else {
                None
            }
        })
        .collect()
}

pub fn match_patterns(
    deposits: &Vec<Deposit>,
    withdraws: &Vec<Withdraw>,
) -> Vec<(H160, H160, DepositWithdrawPattern)> {
    let depositors = deposits
        .iter()
        // get all depositors without duplicates
        .fold(vec![], |addrs, d| {
            if !addrs.contains(&d.from) {
                immut_append!(addrs, d.from)
            } else {
                addrs
            }
        });

    let withdrawers = withdraws
        .iter()
        // get all depositors without duplicates
        .fold(vec![], |addrs, d| {
            if !addrs.contains(&d.receiver) {
                immut_append!(addrs, d.receiver)
            } else {
                addrs
            }
        });

    let deposit_patterns: Vec<(H160, DepositWithdrawPattern)> = depositors
        .iter()
        .map(|a| {
            (*a, {
                deposits
                    .iter()
                    .fold(
                        (0u64, 0u64, 0u64, 0u64),
                        |(_0_1eth, _1eth, _10eth, _100eth), d| {
                            if *a == d.from {
                                match d.pool {
                                    Pool::_0_1ETH => (_0_1eth + 1, _1eth, _10eth, _100eth),
                                    Pool::_1ETH => (_0_1eth, _1eth + 1, _10eth, _100eth),
                                    Pool::_10ETH => (_0_1eth, _1eth, _10eth + 1, _100eth),
                                    Pool::_100ETH => (_0_1eth, _1eth, _10eth, _100eth + 1),
                                    Pool::Unknown => (_0_1eth, _1eth, _10eth, _100eth),
                                }
                            } else {
                                (_0_1eth, _1eth, _10eth, _100eth)
                            }
                        },
                    )
                    .into()
            })
        })
        .collect();

    let withdraw_patterns: Vec<(H160, DepositWithdrawPattern)> = withdrawers
        .iter()
        .map(|a| {
            (*a, {
                withdraws
                    .iter()
                    .fold(
                        (0u64, 0u64, 0u64, 0u64),
                        |(_0_1eth, _1eth, _10eth, _100eth), w| {
                            if *a == w.receiver {
                                match w.pool {
                                    Pool::_0_1ETH => (_0_1eth + 1, _1eth, _10eth, _100eth),
                                    Pool::_1ETH => (_0_1eth, _1eth + 1, _10eth, _100eth),
                                    Pool::_10ETH => (_0_1eth, _1eth, _10eth + 1, _100eth),
                                    Pool::_100ETH => (_0_1eth, _1eth, _10eth, _100eth + 1),
                                    Pool::Unknown => (_0_1eth, _1eth, _10eth, _100eth),
                                }
                            } else {
                                (_0_1eth, _1eth, _10eth, _100eth)
                            }
                        },
                    )
                    .into()
            })
        })
        .collect();

    let pattern_matches: Vec<(H160, H160, &DepositWithdrawPattern, &DepositWithdrawPattern)> =
        deposit_patterns
            .iter()
            .filter(|(_, dp)| pattern_is_interesting(&dp))
            .map(|(a, dp)| {
                withdraw_patterns
                    .iter()
                    .filter_map(move |(a_, wp)| if wp == dp { Some((a_, wp)) } else { None })
                    .map(move |(a_, wp)| (*a, *a_, wp, dp))
            })
            .flatten()
            .collect();

    pattern_matches
        .iter()
        .filter(|(_, _, dp, _)| {
            pattern_matches
                .iter()
                .filter(|(_, _, dp_, _)| dp == dp_)
                .count()
                == 1
        })
        .map(move |(da, wa, p, _)| (*da, *wa, **p))
        .collect()
}
