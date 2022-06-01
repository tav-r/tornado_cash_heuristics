use super::DepositWithdrawPattern;
use crate::data::{Deposit, Withdraw};
use crate::helpers::collect_pools;
use crate::immut_append;
use itertools::Itertools;
use std::collections::HashMap;
use web3::types::H160;

// This function is not strictly needed - it is used in the deposit/withdraw pattern
// finding function (match_pattern) to make the algorithm faster by pre-filtering
// patterns.
fn pattern_is_interesting(pattern: &DepositWithdrawPattern) -> bool {
    // address has deposited more than once to the same pool...
    (pattern.n0_1ETH > 1 || pattern.n1ETH > 1 || pattern.n10ETH > 1 || pattern.n100ETH > 1)
    // ...and address has deposited to multiple different pools
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

// check if each deposit to a pool was made before a withdraw from this pool
fn earlier(deposits: &[&Deposit], withdraws: &[&Withdraw]) -> bool {
    let (d0_1eth, d1eth, d10eth, d100eth) = collect_pools(deposits);
    let (w0_1eth, w1eth, w10eth, w100eth) = collect_pools(withdraws);

    [
        (d0_1eth, w0_1eth),
        (d1eth, w1eth),
        (d10eth, w10eth),
        (d100eth, w100eth),
    ]
    .iter()
    .all(|(ds, ws)| {
        ds.iter()
            .zip(ws.iter())
            .all(|(d, w)| d.block_number <= w.block_number)
    })
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
/// * `deposits` - a slice of references to Deposit structures to scan
/// * `withdraws` - a slice of references to Withdraw structures to scan
pub fn get_address_matches<'a>(
    deposits: &[&'a Deposit],
    withdraws: &[&'a Withdraw],
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
                    deposits.iter().filter(|d| d.from == a).copied().collect(),
                    withdraws
                        .iter()
                        .filter(|w| w.receiver == a)
                        .copied()
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
            if !ds.is_empty() && !later_ws.is_empty() {
                Some((a, (ds, later_ws)))
            } else {
                None
            }
        })
        .collect()
}

/// Get a vector of triples of two addresses and a certain (([deposit/withdraw pattern](DepositWithdrawPattern))
/// between both addresses.
///
/// # Arguments
///
/// * `deposits` - a slice of references to Deposit structures to scan
/// * `withdraws` - a slice of references to Withdraw structures to scan
pub fn match_patterns(
    deposits: &[&Deposit],
    withdraws: &[&Withdraw],
) -> Vec<(H160, H160, DepositWithdrawPattern)> {
    // get a mapping from addresses to deposits made by this address
    let depositors: HashMap<_, _> =
        HashMap::from_iter(deposits.iter().map(|d| d.from).unique().map(|a| {
            (
                a,
                deposits
                    .iter()
                    .filter(|d| d.from == a)
                    .copied()
                    .collect::<Vec<&Deposit>>(),
            )
        }));

    // get a mapping from addresses to withdraws made by this address
    let withdrawers: HashMap<_, _> =
        HashMap::from_iter(withdraws.iter().map(|d| d.receiver).unique().map(|a| {
            (
                a,
                withdraws
                    .iter()
                    .filter(|d| d.receiver == a)
                    .copied()
                    .collect::<Vec<&Withdraw>>(),
            )
        }));

    // get deposit patterns
    let deposit_patterns: Vec<(H160, DepositWithdrawPattern)> =
        depositors.iter().map(|(a, ds)| (*a, ds.into())).collect();

    // get withdraw patterns
    let withdraw_patterns: Vec<(H160, DepositWithdrawPattern)> =
        withdrawers.iter().map(|(a, ws)| (*a, ws.into())).collect();

    // search matching patterns
    let pattern_matches: Vec<(
        H160,                    // depositing address
        H160,                    // withdrawing address
        &DepositWithdrawPattern, // deposit pattern
        &DepositWithdrawPattern, // withdraw pattern
    )> = deposit_patterns
        .iter()
        // select only "interesting" patterns
        .filter(|(_, dp)| pattern_is_interesting(dp))
        // for each deposit pattern find all equal withdraw patterns
        .flat_map(|(a, dp)| {
            withdraw_patterns
                .iter()
                .filter_map(move |(a_, wp)| if wp == dp { Some((a_, wp)) } else { None })
                .map(move |(a_, wp)| (*a, *a_, wp, dp))
        })
        .collect();

    pattern_matches
        .iter()
        .filter(|(_, _, dp, _)| {
            // select only matches that are unique
            pattern_matches
                .iter()
                .filter(|(_, _, dp_, _)| dp == dp_)
                .count()
                == 1
        })
        // select only matches for which each deposit to a pool was made _before_ a withdraw from this pool
        .filter(|(da, wa, _, _)| earlier(depositors.get(da).unwrap(), withdrawers.get(wa).unwrap()))
        .map(move |(da, wa, p, _)| (*da, *wa, **p))
        .collect()
}
