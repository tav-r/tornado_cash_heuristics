mod analysis;
mod data;
mod helpers;
mod subcommands;

use analysis::prepare::split_deposit_withdraw;
use clap::{arg, command};
use data::{Deposit, ESNormalTransaction, Withdraw};
use helpers::load_files;
use subcommands::{address_matches, multiple_denomination};

type SubcommandFunction =
    for<'r, 's, 't, 'u> fn(&'r [&'s data::Deposit], &'t [&'u data::Withdraw], bool);

fn main() {
    // "register" available heuristics
    let available_heuristics = [
        ("address_match", address_matches as SubcommandFunction),
        (
            "multiple_denomination",
            multiple_denomination as SubcommandFunction,
        ),
    ];

    // parse command line arguments
    let matches = command!("tornado_cash_heuristics")
        .version("0.1")
        .author("David Herrmann <david.herrmann@protonmail.com>")
        .arg(
            arg!(--heuristics -e ...)
                .help("Comma-separated list of heuristics to use")
                .takes_value(true)
                .use_value_delimiter(true)
                .min_values(1),
        )
        .arg(arg!(--verbose -v ...).help("Print details (e.g., revealing transactions etc.)"))
        .arg(arg!(--list -l ...).help("List available heuristics"))
        .arg(arg!(["files"]).takes_value(true).min_values(1))
        .get_matches();
    let verbose = matches.is_present("verbose");

    // if list of heuristics should be printed, print it and exit
    if matches.is_present("list") {
        println!("The following heuristics are present:\n");
        available_heuristics
            .iter()
            .for_each(|(n, _)| println!("- {}", n));

        return;
    };

    // load and parse transaction history files specified via command line, filter out errors
    let files: Vec<&str> = matches
        .values_of("files")
        .expect("Please supply path(s) to transaction history file(s)")
        .collect();
    let calls: Vec<ESNormalTransaction> =
        load_files(files, &|t: &ESNormalTransaction| t.isError == 0);

    // divide calls into deposits and withdraws, drop other calls, obtain vectors of references to
    // Deposit/Withdraw structs for later use
    let (deposits, withdraws) = split_deposit_withdraw(
        &calls
            .iter()
            .filter(|c| c.to.is_some())
            .collect::<Vec<&ESNormalTransaction>>(),
    );
    let deposit_refs: Vec<&Deposit> = deposits.iter().collect();
    let withdraw_refs: Vec<&Withdraw> = withdraws.iter().collect();

    // run algorithms
    {
        println!(
            "loaded {} deposits, {} withdraws",
            deposits.len(),
            withdraws.len(),
        );

        // if heuristics were selected, apply them
        if let Some(heuristics) = matches.values_of("heuristics") {
            let heuristics_vec = heuristics.collect::<Vec<&str>>();

            available_heuristics.iter().for_each(|(name, f)| {
                if heuristics_vec.contains(name) {
                    f(&deposit_refs, &withdraw_refs, verbose)
                } else {
                }
            });
        // otherwise apply all
        } else {
            available_heuristics
                .iter()
                .for_each(|(_, f)| f(&deposit_refs, &withdraw_refs, verbose))
        }
    }
}
