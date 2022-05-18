use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::fmt::Debug;

pub fn parse_file<'a, T, U: TryInto<T> + Debug + Serialize + Deserialize<'a>>(
    contents: &'a str,
) -> Vec<T> {
    from_str::<Vec<U>>(&contents)
        .unwrap()
        .into_iter()
        .map(|ess| ess.try_into().or(Err(())))
        .map(|r| r.unwrap())
        .collect()
}
