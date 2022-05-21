use crate::data::{Deposit, ESNormalTransaction, RouterCall, Withdraw};

pub fn split_deposit_withdraw<'a>(
    calls: &Vec<&'a ESNormalTransaction>,
) -> (Vec<Deposit>, Vec<Withdraw>) {
    calls.iter().fold((vec![], vec![]), |(dep, wit), c| {
        let rc: RouterCall = c.input.as_ref().unwrap()[..].into();
        match rc {
            RouterCall::Withdraw(w) => {
                let rec: &[u8; 20] = w._recipient[..].try_into().unwrap();
                let rel: &[u8; 20] = w._relayer[..].try_into().unwrap();
                (
                    dep,
                    wit.into_iter()
                        .chain(
                            [Withdraw::new(
                                c.hash,
                                c.blockNumber,
                                rec.into(),
                                rel.into(),
                                w._fee,
                            )]
                            .into_iter(),
                        )
                        .collect(),
                )
            }
            RouterCall::Deposit(_) => (
                dep.into_iter()
                    .chain([Deposit::new(c.hash, c.blockNumber, c.from)].into_iter())
                    .collect(),
                wit,
            ),
            _ => (dep, wit),
        }
    })
}
