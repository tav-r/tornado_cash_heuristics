use crate::analysis::analyze::{get_address_matches, match_patterns};
use crate::data::{Deposit, Pool, Withdraw};
use crate::immut_append;

fn put_into_pool<'a, T>(
    _0_1eth: Vec<&'a T>,
    _1eth: Vec<&'a T>,
    _10eth: Vec<&'a T>,
    _100eth: Vec<&'a T>,
    pool: &Pool,
    t: &'a T,
) -> (Vec<&'a T>, Vec<&'a T>, Vec<&'a T>, Vec<&'a T>) {
    match pool {
        Pool::_0_1ETH => (immut_append!(_0_1eth, t), _1eth, _10eth, _100eth),
        Pool::_1ETH => (_0_1eth, immut_append!(_1eth, t), _10eth, _100eth),
        Pool::_10ETH => (_0_1eth, _1eth, immut_append!(_10eth, t), _100eth),
        Pool::_100ETH => (_0_1eth, _1eth, _10eth, immut_append!(_100eth, t)),
        _ => (_0_1eth, _1eth, _10eth, _100eth),
    }
}

fn get_withdraws_by_pool(
    withdraws: Vec<&'_ Withdraw>,
) -> (
    Vec<&'_ Withdraw>,
    Vec<&'_ Withdraw>,
    Vec<&'_ Withdraw>,
    Vec<&'_ Withdraw>,
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

fn get_deposits_by_pool(
    deposits: Vec<&'_ Deposit>,
) -> (
    Vec<&'_ Deposit>,
    Vec<&'_ Deposit>,
    Vec<&'_ Deposit>,
    Vec<&'_ Deposit>,
) {
    deposits.iter().fold(
        (vec![], vec![], vec![], vec![]),
        |(_0_1, _1, _10, _100): (Vec<&Deposit>, Vec<&Deposit>, Vec<&Deposit>, Vec<&Deposit>), d| {
            put_into_pool(_0_1, _1, _10, _100, &d.pool, d)
        },
    )
}

pub fn address_matches(deposits: &[Deposit], withdraws: &[Withdraw]) {
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
    }
}

pub fn multiple_denomination(deposits: &[Deposit], withdraws: &[Withdraw]) {
    let res = match_patterns(deposits, withdraws);
    println!("{} unique deposit/withdraw patterns found", res.len());
    dbg!(res);
}
