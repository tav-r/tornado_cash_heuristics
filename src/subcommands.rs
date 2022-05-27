use crate::analysis::analyze::{get_address_matches, match_patterns};
use crate::data::{Deposit, Pool, Withdraw};
use crate::immut_append;

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

pub fn address_matches(deposits: &Vec<Deposit>, withdraws: &Vec<Withdraw>) {
    // address matches per pool
    // get deposits and withdraws by pool
    let (dep_0_1_eth, dep_1_eth, dep_10_eth, dep_100_eth) =
        get_deposits_by_pool(deposits.iter().collect());

    let (withd_0_1_eth, withd_1_eth, withd_10_eth, withd_100_eth) =
        get_withdraws_by_pool(withdraws.iter().collect());

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
        // print_results(res);
    }
}

pub fn multiple_denomination(deposits: &Vec<Deposit>, withdraws: &Vec<Withdraw>) {
    println!(
        "{} unique deposit/withdraw patterns found",
        match_patterns(deposits, withdraws).len()
    );
}
