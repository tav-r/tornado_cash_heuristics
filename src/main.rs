mod analysis;
mod data;
mod helpers;
mod subcommands;

use analysis::prepare::split_deposit_withdraw;
use clap::{Arg, Command};
use data::ESNormalTransaction;
use helpers::load_files;
use subcommands::{address_matches, multiple_denomination};

fn main() {
    let matches = Command::new("tornado_cash_heuristics")
        .version("0.1")
        .author("David Herrmann <david.herrmann@protonmail.com>")
        .arg(Arg::new("files").min_values(1))
        .get_matches();

    let files: Vec<&str> = matches
        .values_of("files")
        .expect("Please supply path(s) to transaction history files")
        .collect();

    // read and parse transaction histories from files specified via command line, filter errors
    let mut calls: Vec<ESNormalTransaction> =
        load_files(files, &|t: &ESNormalTransaction| t.isError == 0);

    // make sure transactions are sorted by timestamp
    calls.sort_by(|t, t_| t.timeStamp.cmp(&t_.timeStamp));

    // divide calls into deposits and withdraws, drop other calls
    let (deposits, withdraws) = split_deposit_withdraw(
        &calls
            .iter()
            .filter(|c| c.to.is_some())
            .collect::<Vec<&ESNormalTransaction>>(),
    );

    // count deposits and withdraws
    println!(
        "loaded {} deposits, {} withdraws",
        deposits.len(),
        withdraws.len(),
    );

    // address matches
    address_matches(&deposits, &withdraws);

    // multiple denomination
    multiple_denomination(&deposits, &withdraws)

    // unique gas price

    // linked addresses
}
