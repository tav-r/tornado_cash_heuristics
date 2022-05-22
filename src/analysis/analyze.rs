use crate::data::{Deposit, Withdraw};
use std::collections::HashMap;
use web3::types::H160;

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
