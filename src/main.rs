mod analysis;
mod data;
mod helpers;

use std::fs::read_to_string;

use analysis::prepare::split_deposit_withdraw;

// const TORNADO_CASH_1ETH: [u8; 20] = hex!("47ce0c6ed5b0ce3d3a51fdb1c52dc66a7c3c2936");

use data::{Deposit, ESNormalTransaction, ESNormalTransactionStrings, Pool, Withdraw};

use helpers::parse_file;

fn put_into_pool<'a, T>(
    _0_1: Vec<&'a T>,
    _1: Vec<&'a T>,
    _10: Vec<&'a T>,
    _100: Vec<&'a T>,
    pool: &Pool,
    t: &'a T,
) -> (Vec<&'a T>, Vec<&'a T>, Vec<&'a T>, Vec<&'a T>) {
    match pool {
        Pool::_0_1ETH => (immut_append!(_0_1, t), _1, _10, _100),
        Pool::_1ETH => (_0_1, immut_append!(_1, t), _10, _100),
        Pool::_10ETH => (_0_1, _1, immut_append!(_10, t), _100),
        Pool::_100ETH => (_0_1, _1, _10, immut_append!(_100, t)),
        _ => (_0_1, _1, _10, _100),
    }
}

fn get_withdraws_by_pool<'a>(
    withdraws: Vec<&'a Withdraw>,
) -> (
    Vec<&'a Withdraw>,
    Vec<&'a Withdraw>,
    Vec<&'a Withdraw>,
    Vec<&'a Withdraw>,
) {
    withdraws.iter().fold(
        (vec![], vec![], vec![], vec![]),
        |(_0_1, _1, _10, _100): (
            Vec<&Withdraw>,
            Vec<&Withdraw>,
            Vec<&Withdraw>,
            Vec<&Withdraw>,
        ),
         w| put_into_pool(_0_1, _1, _10, _100, &w.pool, w),
    )
}

fn get_deposits_by_pool<'a>(
    deposits: Vec<&'a Deposit>,
) -> (
    Vec<&'a Deposit>,
    Vec<&'a Deposit>,
    Vec<&'a Deposit>,
    Vec<&'a Deposit>,
) {
    deposits.iter().fold(
        (vec![], vec![], vec![], vec![]),
        |(_0_1, _1, _10, _100): (Vec<&Deposit>, Vec<&Deposit>, Vec<&Deposit>, Vec<&Deposit>), d| {
            put_into_pool(_0_1, _1, _10, _100, &d.pool, d)
        },
    )
}

fn print_results(
    res: std::collections::HashMap<
        web3::types::H160,
        (
            std::vec::Vec<&data::Deposit>,
            std::vec::Vec<&data::Withdraw>,
        ),
    >,
) {
    res.iter().for_each(|(addr, (dep, wit))| {
        println!(
            "address {}: {} deposits (blocks {}), and {} withdraws (blocks {})",
            hashstring!(addr),
            dep.len(),
            dep.into_iter()
                .map(|d| format!("{}", d.block_number))
                .collect::<Vec<String>>()
                .join(", "),
            wit.len(),
            wit.into_iter()
                .map(|w| format!("{}", w.block_number))
                .collect::<Vec<String>>()
                .join(", ")
        )
    });
}

fn main() {
    // read and parse history of "normal" transactions (calls made by EOAs)
    let router_history = read_to_string("history_router").unwrap();
    let eth0_1_history = read_to_string("history_0.1ETH").unwrap();
    let eth1_history = read_to_string("history_1ETH").unwrap();
    let eth10_history = read_to_string("history_10ETH").unwrap();
    let eth100_history = read_to_string("history_100ETH").unwrap();

    let mut calls: Vec<ESNormalTransaction> = parse_file::<
        ESNormalTransaction,
        ESNormalTransactionStrings,
    >(&router_history)
    .into_iter()
    .chain(
        parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(&eth0_1_history).into_iter(),
    )
    .chain(parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(&eth1_history).into_iter())
    .chain(
        parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(&eth10_history).into_iter(),
    )
    .chain(
        parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(&eth100_history).into_iter(),
    )
    .filter(|t| t.isError == 0)
    .collect();

    // make sure transactions are sorted by timestamp
    calls.sort_by(|t, t_| t.timeStamp.cmp(&t_.timeStamp));

    // divide calls into deposits and withdraws, drop other calls
    let (deposits, withdraws) =
        split_deposit_withdraw(&calls.iter().filter(|c| c.to.is_some()).collect());

    println!(
        "loaded {} deposits, {} withdraws",
        deposits.len(),
        withdraws.len(),
    );

    // get deposits and withdraws by pool
    let (dep_0_1_eth, dep_1_eth, dep_10_eth, dep_100_eth) =
        get_deposits_by_pool(deposits.iter().collect());

    let (withd_0_1_eth, withd_1_eth, withd_10_eth, withd_100_eth) =
        get_withdraws_by_pool(withdraws.iter().collect());

    // address matches per pool
    for (p, (d, w)) in ["0.1 ETH", "1 ETH", "10 ETH", "100 Eth"].into_iter().zip(
        [dep_0_1_eth, dep_1_eth, dep_10_eth, dep_100_eth]
            .into_iter()
            .zip([withd_0_1_eth, withd_1_eth, withd_10_eth, withd_100_eth]),
    ) {
        println!("[*] {} pool", p);
        println!("Analysing {} deposits and {} withdraws", d.len(), w.len());
        let res = analysis::analyze::get_address_matches(&d, &w);
        println!("{} potentially compromised addresses", res.len());
        print_results(res);
    }

    // unique gas price

    // linked addresses

    // multiple denomination
}
