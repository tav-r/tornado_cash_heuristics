use crate::data::{Deposit, Withdraw};

pub fn get_address_matches<'a>(
    deposits: &'a Vec<Deposit>,
    withdraws: &'a Vec<Withdraw>,
) -> Vec<(&'a Deposit, &'a Withdraw)> {
    deposits
        .iter()
        .map(|d| {
            withdraws
                .iter()
                .filter(|w| w.receiver == w.relayer)
                .filter_map(move |w| {
                    if w.block_number >= d.block_number && w.receiver == d.from {
                        Some((d, w))
                    } else {
                        None
                    }
                })
        })
        .flatten()
        .collect()
}
