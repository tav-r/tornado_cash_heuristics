use super::TORNADO_CASH_ROUTER;
use crate::data::{Deposit, ESNormalTransaction, PoolCall, RouterCall, Withdraw};

fn router_call(
    call: &ESNormalTransaction,
    dep: Vec<Deposit>,
    wit: Vec<Withdraw>,
) -> (Vec<Deposit>, Vec<Withdraw>) {
    let rc: RouterCall = call.input.as_ref().unwrap()[..].into();

    match rc {
        RouterCall::Withdraw(w) => {
            let rec: &[u8; 20] = w._recipient[..].try_into().unwrap();
            let rel: &[u8; 20] = w._relayer[..].try_into().unwrap();
            let pool_addr: &[u8; 20] = w._tornado[..].try_into().unwrap();

            (
                dep,
                wit.into_iter()
                    .chain(
                        [Withdraw::new(
                            call.hash,
                            call.blockNumber,
                            pool_addr.into(),
                            rec.into(),
                            rel.into(),
                            w._fee,
                        )]
                        .into_iter(),
                    )
                    .collect(),
            )
        }
        RouterCall::Deposit(d) => {
            let pool_addr: &[u8; 20] = d._tornado[..].try_into().unwrap();
            (
                dep.into_iter()
                    .chain(
                        [Deposit::new(
                            call.hash,
                            call.blockNumber,
                            pool_addr.into(),
                            call.from,
                        )]
                        .into_iter(),
                    )
                    .collect(),
                wit,
            )
        }
        _ => (dep, wit),
    }
}

fn pool_call(
    call: &ESNormalTransaction,
    dep: Vec<Deposit>,
    wit: Vec<Withdraw>,
) -> (Vec<Deposit>, Vec<Withdraw>) {
    let rc: PoolCall = call.input.as_ref().unwrap()[..].into();

    match rc {
        PoolCall::Withdraw(w) => {
            let rec: &[u8; 20] = w._recipient[..].try_into().unwrap();
            let rel: &[u8; 20] = w._relayer[..].try_into().unwrap();
            (
                dep,
                wit.into_iter()
                    .chain(
                        [Withdraw::new(
                            call.hash,
                            call.blockNumber,
                            call.to.unwrap(),
                            rec.into(),
                            rel.into(),
                            w._fee,
                        )]
                        .into_iter(),
                    )
                    .collect(),
            )
        }
        PoolCall::Deposit(_) => (
            dep.into_iter()
                .chain(
                    [Deposit::new(
                        call.hash,
                        call.blockNumber,
                        call.to.unwrap(),
                        call.from,
                    )]
                    .into_iter(),
                )
                .collect(),
            wit,
        ),
        _ => (dep, wit),
    }
}

/// Parse transactions contract function calls. If the input of a call should be (tried)
/// to be parsed as a call to a router or as a "direct" call to a pool is decided by
/// checking if the receiver has the Tornado Cash Router address or not.
///
/// # Arguments
/// * calls - a reference to a vector of references to ESNormalTransaction structs (which represent result entries obtained from Etherscan)
pub fn split_deposit_withdraw<'a>(
    calls: &Vec<&'a ESNormalTransaction>,
) -> (Vec<Deposit>, Vec<Withdraw>) {
    calls.iter().fold((vec![], vec![]), |(dep, wit), c| {
        if c.to.unwrap() == TORNADO_CASH_ROUTER.into() {
            router_call(c, dep, wit)
        } else {
            pool_call(c, dep, wit)
        }
    })
}
