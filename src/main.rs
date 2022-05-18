mod analysis;
mod data;
mod helpers;
use std::fs::read_to_string;

use analysis::{
    prepare::{divide_to_from, divide_withdraw_deposit, group_fee_withdraws},
    Withdraw,
};

use data::{
    ESInternalTransaction, ESInternalTransactionStrings, ESNormalTransaction,
    ESNormalTransactionStrings,
};

use helpers::parse_file;

fn main() {
    // read and parse history of internal transactions (forwarded by router)
    let internal_history = read_to_string("history").unwrap();
    let mut internal_transactions =
        parse_file::<ESInternalTransaction, ESInternalTransactionStrings>(&internal_history);

    // make sure transactions are sorted by timestamp (even though they most likely already are)
    internal_transactions.sort_by(|t, t_| t.timeStamp.cmp(&t_.timeStamp));

    // divide transactions into sent and received transactions
    let (internal_received, internal_sent) = divide_to_from(&internal_transactions);

    // the sent transactions could be to send fees or to send deposited coins, group fees and matching
    // transfers to receivers
    let internal_withdraws = group_fee_withdraws(internal_sent).unwrap();

    // some sanity checks
    // 1. all received transactions had a value of 1 ETH
    assert!(internal_received
        .iter()
        .all(|t| t.value == 1_000_000_000_000_000_000 || t.isError != 0));
    // 2. all withdrawals had a total sum of 1 ETH
    assert!(internal_withdraws.iter().all(|t| match t {
        Withdraw::WithoutRelayer(t) => t.value == 1_000_000_000_000_000_000 || t.isError != 0,
        Withdraw::WithRelayer(t, f) =>
            t.value + f.value == 1_000_000_000_000_000_000 || t.isError != 0,
    }));

    // print some information about the parsed/prepared internal transactions
    println!(
        "parsed {} deposits and {} withdraws ({} withdraws without relayer), {} withdraws with errors",
        internal_received.len(),
        internal_withdraws.len(),
        internal_withdraws
            .iter()
            .filter(|w| match w {
                Withdraw::WithoutRelayer(_) => true,
                _ => false,
            })
            .count(),
        internal_withdraws
            .iter()
            .filter(|w| match w {
                Withdraw::WithoutRelayer(d) => d.isError != 0,
                Withdraw::WithRelayer(a, b) => a.isError + b.isError != 0,
            })
            .count()
    );

    // read and parse history of "normal" transactions (calls made by EOAs)
    let normal_history = read_to_string("history_ext").unwrap();
    let mut normal_transactions =
        parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(&normal_history);

    // make sure transactions are sorted by timestamp
    normal_transactions.sort_by(|t, t_| t.timeStamp.cmp(&t_.timeStamp));

    // Divide transactions into those that have "to" set to the contract and those that have not.
    // The only transaction that was not to the contract was its creation
    let (_, normal_received) = divide_to_from(&normal_transactions);
    assert_eq!(normal_received.len(), 1);
    println!(
        "Contract deployment: {}",
        hashstring!(normal_received.get(0).unwrap().hash)
    );

    // divide calls into deposit, withdraw and other calls
    let (normal_deposits, normal_withdraws, others) = divide_withdraw_deposit(&normal_transactions);

    // print some info
    println!(
        "parsed {} directly sent deposits ({} had errors) and {} directly sent withdraws ({} errors)",
        normal_deposits.len(),
        normal_deposits.iter().filter(|t| t.isError != 0).count(),
        normal_withdraws.len(),
        normal_withdraws.iter().filter(|t| t.isError != 0).count(),
    );

    // address match

    // unique gas price

    // linked addresses

    // multiple denomination
}
