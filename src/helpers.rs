use crate::data::{ESNormalTransaction, ESNormalTransactionStrings};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::Debug;
use std::fs::read_to_string;

fn parse_file<'a, T, U: TryInto<T> + Debug + Serialize + Deserialize<'a>>(
    contents: &'a str,
) -> Vec<T> {
    from_str::<Vec<U>>(&contents)
        .unwrap()
        .into_iter()
        .map(|ess| ess.try_into().or(Err(())))
        .map(|r| r.unwrap())
        .collect()
}

pub fn load_files(
    paths: Vec<&str>,
    filter: &dyn Fn(&ESNormalTransaction) -> bool,
) -> Vec<ESNormalTransaction> {
    paths
        .into_iter()
        .map(|p| {
            parse_file::<ESNormalTransaction, ESNormalTransactionStrings>(
                &read_to_string(p).expect(&format!("could not read file '{}'", p)),
            )
            .into_iter()
        })
        .flatten()
        .filter(|t| filter(t))
        .collect()
}
