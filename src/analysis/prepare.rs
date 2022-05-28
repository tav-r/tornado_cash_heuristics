use super::TORNADO_CASH_ROUTER;
use crate::data::{Deposit, ESNormalTransaction, PoolCall, RouterCall, Withdraw};

fn router_call(
    call: &ESNormalTransaction,
    dep: Vec<Deposit>,
    wit: Vec<Withdraw>,
) -> (Vec<Deposit>, Vec<Withdraw>) {
    let rc: RouterCall = call.input.as_ref().unwrap()[..].into();

    match rc {
        RouterCall::Withdraw(w) => (
            dep,
            immut_append!(
                wit,
                Withdraw::new(
                    call.hash,
                    call.blockNumber,
                    w._tornado,
                    w._recipient,
                    w._relayer,
                    w._fee,
                )
            ),
        ),
        RouterCall::Deposit(d) => {
            let pool_addr: &[u8; 20] = d._tornado[..].try_into().unwrap();

            (
                immut_append!(
                    dep,
                    Deposit::new(call.hash, call.blockNumber, pool_addr.into(), call.from,)
                ),
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
        PoolCall::Withdraw(w) => (
            dep,
            immut_append!(
                wit,
                Withdraw::new(
                    call.hash,
                    call.blockNumber,
                    call.to.unwrap(),
                    w._recipient,
                    w._relayer,
                    w._fee,
                )
            ),
        ),
        PoolCall::Deposit(_) => (
            immut_append!(
                dep,
                Deposit::new(call.hash, call.blockNumber, call.to.unwrap(), call.from,)
            ),
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
pub fn split_deposit_withdraw(calls: &[&ESNormalTransaction]) -> (Vec<Deposit>, Vec<Withdraw>) {
    calls.iter().fold((vec![], vec![]), |(dep, wit), c| {
        if c.to.unwrap() == TORNADO_CASH_ROUTER.into() {
            router_call(c, dep, wit)
        } else {
            pool_call(c, dep, wit)
        }
    })
}
