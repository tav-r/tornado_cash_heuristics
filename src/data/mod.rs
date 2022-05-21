mod implementations;

use ethabi::{decode, param_type::ParamType, short_signature, Address as EthtypeAddress, Uint};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use web3::types::{H160, H256};

pub trait ESTransaction {
    fn transaction_hash(&self) -> H256;
    fn transaction_value(&self) -> u128;
    fn transaction_from(&self) -> H160;
    fn transaction_to(&self) -> Option<H160>;
    fn transaction_blocknumber(&self) -> u128;
    fn transaction_is_error(&self) -> u128;
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

const WITHDRAW_SIGNATURE: [ParamType; 8] = [
    ParamType::Address,
    ParamType::Bytes,
    ParamType::FixedBytes(32),
    ParamType::FixedBytes(32),
    ParamType::Address,
    ParamType::Address,
    ParamType::Uint(256),
    ParamType::Uint(256),
];

const DEPOSIT_SIGNATURE: [ParamType; 3] = [
    ParamType::Address,
    ParamType::FixedBytes(32),
    ParamType::Bytes,
];

pub enum RouterCall {
    Withdraw(Withdraw),
    Deposit(Deposit),
    Other,
}

impl Into<RouterCall> for &[u8] {
    fn into(self) -> RouterCall {
        if self[0..4] == short_signature("withdraw", &WITHDRAW_SIGNATURE) {
            RouterCall::Withdraw(self[4..].try_into().unwrap())
        } else if self[0..4] == short_signature("deposit", &DEPOSIT_SIGNATURE) {
            RouterCall::Deposit(self[4..].try_into().unwrap())
        } else {
            RouterCall::Other
        }
    }
}

#[allow(non_snake_case)]
pub struct Withdraw {
    pub _tornado: EthtypeAddress,
    pub _proof: Vec<u8>,
    pub _root: Vec<u8>,
    pub _nullifierHash: Vec<u8>,
    pub _recipient: EthtypeAddress,
    pub _relayer: EthtypeAddress,
    pub _fee: Uint,
    pub _refund: Uint,
}

#[allow(non_snake_case)]
pub struct Deposit {
    pub _tornado: EthtypeAddress,
    pub _commitment: Vec<u8>,
    pub _encryptedNote: Vec<u8>,
}

impl TryInto<Deposit> for &[u8] {
    type Error = ();
    fn try_into(self) -> Result<Deposit, ()> {
        if let Ok(v) = decode(&DEPOSIT_SIGNATURE, &self[..]) {
            Ok(Deposit {
                _tornado: v[0].clone().into_address().unwrap(),
                _commitment: v[1].clone().into_fixed_bytes().unwrap(),
                _encryptedNote: v[2].clone().into_bytes().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

impl TryInto<Withdraw> for &[u8] {
    type Error = ();
    fn try_into(self) -> Result<Withdraw, ()> {
        if let Ok(v) = decode(&WITHDRAW_SIGNATURE, &self[..]) {
            Ok(Withdraw {
                _tornado: v[0].clone().into_address().unwrap(),
                _proof: v[1].clone().into_bytes().unwrap(),
                _root: v[2].clone().into_fixed_bytes().unwrap(),
                _nullifierHash: v[3].clone().into_fixed_bytes().unwrap(),
                _recipient: v[4].clone().into_address().unwrap(),
                _relayer: v[5].clone().into_address().unwrap(),
                _fee: v[6].clone().into_uint().unwrap(),
                _refund: v[7].clone().into_uint().unwrap(),
            })
        } else {
            Err(())
        }
    }
}
