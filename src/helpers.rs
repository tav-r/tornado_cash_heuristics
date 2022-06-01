use crate::data::{ESNormalTransaction, ESNormalTransactionStrings, InBlock, InPool, Pool};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::Debug;
use std::fs::read_to_string;

/// Turn a struct that implement AsBytes into a hexadecimal number
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

/// Add an element to a struct that implements IntoIter
#[macro_export]
macro_rules! immut_append {
    ($l:expr, $a:expr) => {
        $l.into_iter().chain([$a].into_iter()).collect()
    };
}

// Generic function to parse &str into Vec<T>, used by load_files(...) to load transaction data
fn parse_file<'a, T, U: TryInto<T> + Debug + Serialize + Deserialize<'a>>(
    contents: &'a str,
) -> Vec<T> {
    from_str::<Vec<U>>(contents)
        .unwrap()
        .into_iter()
        .map(|ess| ess.try_into().or(Err(())))
        .map(|r| r.unwrap())
        .collect()
}

/// Load transactions from JSON files obtained from Etherscan API.
///
/// # Arguments
///
/// * paths - vector of strings describing file system paths
/// * filter - a filter function to select transactions with certain properties
pub fn load_files(
    paths: Vec<&str>,
    filter: &dyn Fn(&ESNormalTransaction) -> bool,
) -> Vec<ESNormalTransaction> {
    paths
        .into_iter()
        .flat_map(|p| {
            parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(
                &read_to_string(p).unwrap_or_else(|_| panic!("could not read file '{}'", p)),
            )
            .into_iter()
        })
        .filter(|t| filter(t))
        // get rid of duplicate entries
        .unique()
        .collect()
}

/// Put withdraws/withdraws into separate vectors for each pool, i.e., return a 4-tuple with
/// transactions for the 0.1 ETH pool, the 1 ETH pool, the 10 ETH pool and the 100 ETH pool
/// respectively. Each vector is sorted by the number of the block containing the transaction.
///
/// # Arguments
///
/// * ts - reference to slice of references to struct which implements InPool and InBlock
///        (which holds for Deposits and Withdraws)
pub fn collect_pools<'a, T: InPool + InBlock>(
    ts: &[&'a T],
) -> (Vec<&'a T>, Vec<&'a T>, Vec<&'a T>, Vec<&'a T>) {
    ts.iter().copied().fold(
        (vec![], vec![], vec![], vec![]),
        |(_0_1eth, _1eth, _10eth, _100eth), t| match t.pool() {
            Pool::_0_1ETH => (
                _0_1eth
                    .into_iter()
                    .chain([t].into_iter())
                    .sorted_by_key(|t| t.block())
                    .collect(),
                _1eth,
                _10eth,
                _100eth,
            ),
            Pool::_1ETH => (
                _0_1eth,
                _1eth
                    .into_iter()
                    .chain([t].into_iter())
                    .sorted_by_key(|t| t.block())
                    .collect(),
                _10eth,
                _100eth,
            ),
            Pool::_10ETH => (
                _0_1eth,
                _1eth,
                _10eth
                    .into_iter()
                    .chain([t].into_iter())
                    .sorted_by_key(|t| t.block())
                    .collect(),
                _100eth,
            ),
            Pool::_100ETH => (
                _0_1eth,
                _1eth,
                _10eth,
                _100eth
                    .into_iter()
                    .chain([t].into_iter())
                    .sorted_by_key(|t| t.block())
                    .collect(),
            ),
            Pool::Unknown => (_0_1eth, _1eth, _10eth, _100eth),
        },
    )
}
