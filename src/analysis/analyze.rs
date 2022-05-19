use super::ESTransaction;

pub fn get_address_matches<'a, T: ESTransaction>(
    deposits: &Vec<&'a T>,
    withdraws: &Vec<&'a T>,
) -> Vec<(&'a T, &'a T)> {
    deposits
        .iter()
        .map(|d| {
            withdraws
                .iter()
                .filter(|w| {
                    w.transaction_blocknumber() >= d.transaction_blocknumber()
                        && d.transaction_from() == w.transaction_to().unwrap()
                })
                .map(|w| (*d, *w))
        })
        .flatten()
        .collect()
}
