mod analysis;
mod data;
mod helpers;

use hex_literal::hex;
use std::fs::read_to_string;

use analysis::prepare::split_deposit_withdraw;

// const TORNADO_CASH_1ETH: [u8; 20] = hex!("47ce0c6ed5b0ce3d3a51fdb1c52dc66a7c3c2936");
const TORNADO_CASH_ROUTER: [u8; 20] = hex!("d90e2f925DA726b50C4Ed8D0Fb90Ad053324F31b");

use data::{ESNormalTransaction, ESNormalTransactionStrings};

use helpers::parse_file;

fn main() {
    // read and parse history of "normal" transactions (calls made by EOAs)
    let normal_history = read_to_string("history_ext_router").unwrap();
    let mut normal_transactions =
        parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(&normal_history);

    // make sure transactions are sorted by timestamp
    normal_transactions.sort_by(|t, t_| t.timeStamp.cmp(&t_.timeStamp));

    let calls: Vec<&ESNormalTransaction> = normal_transactions
        .iter()
        .filter(|t| t.to.unwrap() == TORNADO_CASH_ROUTER.into())
        .collect();

    let (deposits, withdraws) = split_deposit_withdraw(&calls);

    // divide calls into deposit, withdraw and other calls

    println!(
        "loaded {} deposits, {} withdraws",
        deposits.len(),
        withdraws.len(),
    );

    // address matches
    let res = analysis::analyze::get_address_matches(&deposits, &withdraws);

    res.iter().for_each(|(dep, wit)| {
        println!(
            "deposit at {}, withdraw at {}, address: {}",
            hashstring!(dep.transaction_hash),
            hashstring!(wit.transaction_hash),
            hashstring!(dep.from)
        )
    });

    // unique gas price

    // linked addresses

    // multiple denomination
}
