mod implementations;

use ethabi::{param_type::ParamType, Uint};
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use web3::types::{H160, H256};

const TORNADO_CASH_0_1ETH: [u8; 20] = hex!("12D66f87A04A9E220743712cE6d9bB1B5616B8Fc");
const TORNADO_CASH_1ETH: [u8; 20] = hex!("47CE0C6eD5B0Ce3d3A51fdb1C52DC66a7c3c2936");
const TORNADO_CASH_10ETH: [u8; 20] = hex!("910Cbd523D972eb0a6f4cAe4618aD62622b39DbF");
const TORNADO_CASH_100ETH: [u8; 20] = hex!("A160cdAB225685dA1d56aa342Ad8841c3b53f291");

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
#[derive(Debug, PartialEq, Hash, Eq, Clone)]
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

const DIRECT_WITHDRAW_SIGNATURE: [ParamType; 7] = [
    ParamType::Bytes,
    ParamType::FixedBytes(32),
    ParamType::FixedBytes(32),
    ParamType::Address,
    ParamType::Address,
    ParamType::Uint(256),
    ParamType::Uint(256),
];

const DIRECT_DEPOSIT_SIGNATURE: [ParamType; 1] = [ParamType::FixedBytes(32)];

const ROUTER_WITHDRAW_SIGNATURE: [ParamType; 8] = [
    ParamType::Address,
    ParamType::Bytes,
    ParamType::FixedBytes(32),
    ParamType::FixedBytes(32),
    ParamType::Address,
    ParamType::Address,
    ParamType::Uint(256),
    ParamType::Uint(256),
];

const ROUTER_DEPOSIT_SIGNATURE: [ParamType; 3] = [
    ParamType::Address,
    ParamType::FixedBytes(32),
    ParamType::Bytes,
];

pub enum RouterCall {
    Withdraw(RouterWithdraw),
    Deposit(RouterDeposit),
    Other,
}

pub enum PoolCall {
    Withdraw(DirectWithdraw),
    Deposit(DirectDeposit),
    Other,
}

#[allow(non_snake_case)]
pub struct DirectWithdraw {
    pub _proof: Vec<u8>,
    pub _root: Vec<u8>,
    pub _nullifierHash: Vec<u8>,
    pub _recipient: H160,
    pub _relayer: H160,
    pub _fee: Uint,
    pub _refund: Uint,
}

#[allow(non_snake_case)]
pub struct RouterWithdraw {
    pub _tornado: H160,
    pub _proof: Vec<u8>,
    pub _root: Vec<u8>,
    pub _nullifierHash: Vec<u8>,
    pub _recipient: H160,
    pub _relayer: H160,
    pub _fee: Uint,
    pub _refund: Uint,
}

#[allow(non_snake_case)]
pub struct DirectDeposit {
    pub _commitment: Vec<u8>,
}

#[allow(non_snake_case)]
pub struct RouterDeposit {
    pub _tornado: H160,
    pub _commitment: Vec<u8>,
    pub _encryptedNote: Vec<u8>,
}

#[derive(Debug)]
pub struct Withdraw {
    pub transaction_hash: H256,
    pub block_number: u128,
    pub pool: Pool,
    pub receiver: H160,
    pub relayer: H160,
    pub fee: Uint,
}

#[derive(Debug)]
pub struct Deposit {
    pub transaction_hash: H256,
    pub block_number: u128,
    pub pool: Pool,
    pub from: H160,
}

#[derive(Debug)]
pub enum Pool {
    _0_1ETH,
    _1ETH,
    _10ETH,
    _100ETH,
    Unknown,
}

pub trait InPool {
    fn pool(&'_ self) -> &'_ Pool;
}

pub trait InBlock {
    fn block(&self) -> u128;
}
