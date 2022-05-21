mod analysis;
mod data;
mod data_check;
mod helpers;

use hex_literal::hex;
use std::fs::read_to_string;

use analysis::{
    prepare::{divide_to_from, divide_withdraw_deposit, group_fee_withdraws},
    Withdraw,
};

use data::{
    ESInternalTransaction, ESInternalTransactionStrings, ESNormalTransaction,
    ESNormalTransactionStrings, ESTransaction,
};

use helpers::parse_file;

fn count_withdraw_errors<T: ESTransaction>(withdraws: &Vec<Withdraw<T>>) -> usize {
    withdraws
        .iter()
        .filter(|w| match w {
            Withdraw::WithoutRelayer(d) => d.transaction_is_error() != 0,
            Withdraw::WithRelayer(a, b) => a.transaction_is_error() + b.transaction_is_error() != 0,
        })
        .count()
}

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

    assert!(data_check::check_sum_per_deposit(&internal_received));
    assert!(data_check::check_sum_per_withdraw(&internal_withdraws));

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
        count_withdraw_errors(&internal_withdraws)
    );

    // read and parse history of "normal" transactions (calls made by EOAs)
    let normal_history = read_to_string("history_ext").unwrap();
    let mut normal_transactions =
        parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(&normal_history);

    // make sure transactions are sorted by timestamp
    normal_transactions.sort_by(|t, t_| t.timeStamp.cmp(&t_.timeStamp));

    // Divide transactions into those that have "to" set to the contract and those that have not.
    // The only transaction "from" the contract was its creation
    let (_, normal_sent) = divide_to_from(&normal_transactions);
    assert_eq!(normal_sent.len(), 1);

    // divide calls into deposit, withdraw and other calls
    let (normal_deposits, normal_withdraws, _) = divide_withdraw_deposit(&normal_transactions);

    dbg!(internal_received
        .iter()
        .map(|t| t.from == hex!("d90e2f925da726b50c4ed8d0fb90ad053324f31b").into())
        .count());
    // print some info
    println!(
        "parsed {} directly sent deposits ({} had errors) and {} directly sent withdraws ({} errors)",
        normal_deposits.len(),
        normal_deposits.iter().filter(|t| t.isError != 0).count(),
        normal_withdraws.len(),
        normal_withdraws.iter().filter(|t| t.isError != 0).count(),
    );

    // address match
    let check_deposits: Vec<&dyn ESTransaction> = normal_deposits
        .iter()
        .map(|t| *t as &dyn ESTransaction)
        .chain(internal_received.iter().map(|t| *t as &dyn ESTransaction))
        .collect();

    let check_withdraws: Vec<&dyn ESTransaction> = normal_withdraws
        .iter()
        .map(|t| *t as &dyn ESTransaction)
        .chain(internal_withdraws.iter().filter_map(|t| match t {
            Withdraw::WithoutRelayer(_) => None, // filter out direct transactions, they are not considered "errors"
            Withdraw::WithRelayer(_, w) => Some(*w as &dyn ESTransaction),
        }))
        .collect();

    let res: Vec<(&dyn ESTransaction, &dyn ESTransaction)> = check_deposits
        .iter()
        .map(|d| {
            check_withdraws
                .iter()
                .filter(|w| {
                    w.transaction_blocknumber() >= d.transaction_blocknumber()
                        && d.transaction_from() == w.transaction_to().unwrap()
                })
                .map(|w| (*d, *w))
        })
        .flatten()
        .collect();

    res.iter().for_each(|(dep, wit)| {
        println!(
            "deposit at {}, withdraw at {}, address: {}",
            hashstring!(dep.transaction_hash()),
            hashstring!(wit.transaction_hash()),
            hashstring!(dep.transaction_from())
        )
    });

    // unique gas price

    // linked addresses

    // multiple denomination
}
