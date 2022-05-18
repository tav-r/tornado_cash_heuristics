mod implementations;

use serde::{Deserialize, Serialize};
use web3::types::{H160, H256};

pub trait ESTransaction {
    fn transaction_hash(&self) -> H256;
    fn transaction_value(&self) -> u128;
    fn transaction_to(&self) -> Option<H160>;
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ESInternalTransactionStrings {
    blockNumber: String,
    timeStamp: String,
    hash: String,
    from: String,
    to: String,
    value: String,
    contractAddress: String,
    input: String,
    #[serde(rename = "type")]
    type_: String,
    gas: String,
    gasUsed: String,
    traceId: String,
    isError: String,
    errCode: String,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ESInternalTransaction {
    pub blockNumber: u128,
    pub timeStamp: u128,
    pub hash: H256,
    pub from: H160,
    pub to: H160,
    pub value: u128,
    pub contractAddress: Option<H160>,
    pub input: Option<Vec<u8>>,
    pub type_: String,
    pub gas: u128,
    pub gasUsed: u128,
    pub traceId: String,
    pub isError: u128,
    pub errCode: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ESNormalTransactionStrings {
    pub blockNumber: String,
    pub timeStamp: String,
    pub hash: String,
    pub nonce: String,
    pub blockHash: String,
    pub transactionIndex: String,
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas: String,
    pub gasPrice: String,
    pub isError: String,
    pub txreceipt_status: String,
    pub input: String,
    pub contractAddress: String,
    pub cumulativeGasUsed: String,
    pub gasUsed: String,
    pub confirmations: String,
}

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct ESNormalTransaction {
    pub blockNumber: u128,
    pub timeStamp: u128,
    pub hash: H256,
    pub nonce: u128,
    pub blockHash: H256,
    pub transactionIndex: u128,
    pub from: H160,
    pub to: Option<H160>,
    pub value: u128,
    pub gas: u128,
    pub gasPrice: u128,
    pub isError: u128,
    pub txreceipt_status: u128,
    pub input: Option<Vec<u8>>,
    pub contractAddress: Option<H160>,
    pub cumulativeGasUsed: u128,
    pub gasUsed: u128,
    pub confirmations: u128,
}
