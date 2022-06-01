use crate::analysis::analyze::{get_address_matches, match_patterns};
use crate::data::{Deposit, Withdraw};
use crate::hashstring;
use crate::helpers::collect_pools;

/// Find address matches and print results.
pub fn address_matches(deposits: &[&Deposit], withdraws: &[&Withdraw], verbose: bool) {
    // get deposits and withdraws by pool
    let (dep_0_1_eth, dep_1_eth, dep_10_eth, dep_100_eth) = collect_pools(deposits);

    let (withd_0_1_eth, withd_1_eth, withd_10_eth, withd_100_eth) = collect_pools(withdraws);

    for (p, (d, w)) in ["0.1 ETH", "1 ETH", "10 ETH", "100 Eth"].into_iter().zip(
        [dep_0_1_eth, dep_1_eth, dep_10_eth, dep_100_eth]
            .into_iter()
            .zip([withd_0_1_eth, withd_1_eth, withd_10_eth, withd_100_eth]),
    ) {
        let res = get_address_matches(&d, &w);
        println!(
            "{} potentially compromised addresses in the {} pool (analysed {} deposits and {} withdraws)",
            res.len(),
            p,
            d.len(), w.len()
        );

        if verbose {
            res.iter().for_each(|(a, (ds, ws))| {
                println!(
                    "{} deposited at {} and withdrew at {}",
                    hashstring!(a),
                    ds.iter()
                        .map(|d| hashstring!(d.transaction_hash))
                        .collect::<Vec<String>>()
                        .join(", "),
                    ws.iter()
                        .map(|w| hashstring!(w.transaction_hash))
                        .collect::<Vec<String>>()
                        .join(", "),
                )
            })
        }
    }
}

// Find matching deposit/withdraw patterns and print results.
pub fn multiple_denomination(deposits: &[&Deposit], withdraws: &[&Withdraw], verbose: bool) {
    let res = match_patterns(deposits, withdraws);
    println!("{} unique deposit/withdraw patterns found", res.len());

    if verbose {
        res.iter().for_each(|(a, b, _)| {
            println!(
                "{} and {} have the same deposit/withdraw pattern",
                hashstring!(a),
                hashstring!(b)
            )
        });
    }
}
