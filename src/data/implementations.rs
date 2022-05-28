use super::{
    Deposit, DirectDeposit, DirectWithdraw, ESNormalTransaction, ESNormalTransactionStrings,
    ESTransaction, Pool, PoolCall, RouterCall, RouterDeposit, RouterWithdraw, Withdraw,
    DIRECT_DEPOSIT_SIGNATURE, DIRECT_WITHDRAW_SIGNATURE, ROUTER_DEPOSIT_SIGNATURE,
    ROUTER_WITHDRAW_SIGNATURE, TORNADO_CASH_0_1ETH, TORNADO_CASH_100ETH, TORNADO_CASH_10ETH,
    TORNADO_CASH_1ETH,
};
use ethabi::{decode, short_signature, Token, Uint};
use hex::decode as hex_decode;
use std::error::Error;
use web3::types::{H160, H256};

fn token_to_h160(token: &Token) -> Result<H160, Box<dyn Error>> {
    let bytes: [u8; 20] = token.clone().into_address().unwrap()[..].try_into()?;
    Ok(bytes.into())
}

// used by Deposit::new(...) and Withdraw::new(...) to assign the Pool enum
fn pool_by_addr(addr: H160) -> Pool {
    let addr_bytes: [u8; 20] = addr[..].try_into().unwrap();
    match addr_bytes {
        TORNADO_CASH_0_1ETH => Pool::_0_1ETH,
        TORNADO_CASH_1ETH => Pool::_1ETH,
        TORNADO_CASH_10ETH => Pool::_10ETH,
        TORNADO_CASH_100ETH => Pool::_100ETH,
        _ => Pool::Unknown,
    }
}

impl ESTransaction for ESNormalTransaction {
    fn transaction_hash(&self) -> H256 {
        self.hash
    }
    fn transaction_value(&self) -> u128 {
        self.value
    }
    fn transaction_to(&self) -> Option<H160> {
        self.to
    }
    fn transaction_from(&self) -> H160 {
        self.from
    }
    fn transaction_blocknumber(&self) -> u128 {
        self.blockNumber
    }
    fn transaction_is_error(&self) -> u128 {
        self.isError
    }
}

impl TryInto<ESNormalTransaction> for ESNormalTransactionStrings {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<ESNormalTransaction, Box<dyn Error>> {
        Ok(ESNormalTransaction {
            blockNumber: self.blockNumber.parse()?,
            timeStamp: self.timeStamp.parse()?,
            hash: self.hash.get(2..).unwrap().parse()?,
            from: self.from.get(2..).unwrap().parse()?,
            to: if self.to.is_empty() {
                None
            } else {
                Some(self.to.get(2..).unwrap().parse()?)
            },
            value: self.value.parse()?,
            contractAddress: if self.contractAddress.is_empty() {
                None
            } else {
                Some(self.contractAddress.get(2..).unwrap().parse()?)
            },
            input: if self.input.is_empty() {
                None
            } else {
                Some(hex_decode(self.input.get(2..).unwrap())?)
            },
            gas: self.gas.parse()?,
            gasUsed: self.gasUsed.parse()?,
            isError: self.isError.parse()?,
            blockHash: self.blockHash.parse()?,
            confirmations: self.confirmations.parse()?,
            cumulativeGasUsed: self.cumulativeGasUsed.parse()?,
            nonce: self.nonce.parse()?,
            gasPrice: self.gasPrice.parse()?,
            transactionIndex: self.transactionIndex.parse()?,
            txreceipt_status: self.txreceipt_status.parse()?,
        })
    }
}

impl Into<RouterCall> for &[u8] {
    fn into(self) -> RouterCall {
        if self.len() >= 4 && self[0..4] == short_signature("withdraw", &ROUTER_WITHDRAW_SIGNATURE)
        {
            RouterCall::Withdraw(self[4..].try_into().unwrap())
        } else if self.len() >= 4
            && self[0..4] == short_signature("deposit", &ROUTER_DEPOSIT_SIGNATURE)
        {
            RouterCall::Deposit(self[4..].try_into().unwrap())
        } else {
            RouterCall::Other
        }
    }
}

impl TryInto<RouterDeposit> for &[u8] {
    type Error = ();
    fn try_into(self) -> Result<RouterDeposit, ()> {
        if let Ok(v) = decode(&ROUTER_DEPOSIT_SIGNATURE, &self[..]) {
            let pool_addr: [u8; 20] = v[0].clone().into_address().unwrap()[..].try_into().unwrap();

            Ok(RouterDeposit {
                _tornado: pool_addr.into(),
                _commitment: v[1].clone().into_fixed_bytes().unwrap(),
                _encryptedNote: v[2].clone().into_bytes().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

impl TryInto<RouterWithdraw> for &[u8] {
    type Error = Box<dyn Error>;
    fn try_into(self) -> Result<RouterWithdraw, Box<dyn Error>> {
        if let Ok(v) = decode(&ROUTER_WITHDRAW_SIGNATURE, &self[..]) {
            Ok(RouterWithdraw {
                _tornado: token_to_h160(&v[0])?,
                _proof: v[1].clone().into_bytes().unwrap(),
                _root: v[2].clone().into_fixed_bytes().unwrap(),
                _nullifierHash: v[3].clone().into_fixed_bytes().unwrap(),
                _recipient: token_to_h160(&v[4])?,
                _relayer: token_to_h160(&v[5])?,
                _fee: v[6].clone().into_uint().unwrap(),
                _refund: v[7].clone().into_uint().unwrap(),
            })
        } else {
            Err("Could not decode input.".into())
        }
    }
}

impl Into<PoolCall> for &[u8] {
    fn into(self) -> PoolCall {
        if self.len() >= 4 && self[0..4] == short_signature("withdraw", &DIRECT_WITHDRAW_SIGNATURE)
        {
            PoolCall::Withdraw(self[4..].try_into().unwrap())
        } else if self.len() >= 4
            && self[0..4] == short_signature("deposit", &DIRECT_DEPOSIT_SIGNATURE)
        {
            PoolCall::Deposit(self[4..].try_into().unwrap())
        } else {
            PoolCall::Other
        }
    }
}

impl TryInto<DirectDeposit> for &[u8] {
    type Error = ();
    fn try_into(self) -> Result<DirectDeposit, ()> {
        if let Ok(v) = decode(&DIRECT_DEPOSIT_SIGNATURE, &self[..]) {
            Ok(DirectDeposit {
                _commitment: v[0].clone().into_fixed_bytes().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

impl TryInto<DirectWithdraw> for &[u8] {
    type Error = Box<dyn Error>;
    fn try_into(self) -> Result<DirectWithdraw, Box<dyn Error>> {
        if let Ok(v) = decode(&DIRECT_WITHDRAW_SIGNATURE, &self[..]) {
            Ok(DirectWithdraw {
                _proof: v[0].clone().into_bytes().unwrap(),
                _root: v[1].clone().into_fixed_bytes().unwrap(),
                _nullifierHash: v[2].clone().into_fixed_bytes().unwrap(),
                _recipient: token_to_h160(&v[3])?,
                _relayer: token_to_h160(&v[4])?,
                _fee: v[5].clone().into_uint().unwrap(),
                _refund: v[6].clone().into_uint().unwrap(),
            })
        } else {
            Err("Could not decode input.".into())
        }
    }
}

impl Deposit {
    pub fn new(transaction_hash: H256, block_number: u128, pool_address: H160, from: H160) -> Self {
        Self {
            transaction_hash,
            block_number,
            pool: pool_by_addr(pool_address),
            from,
        }
    }
}

impl Withdraw {
    pub fn new(
        transaction_hash: H256,
        block_number: u128,
        pool_address: H160,
        receiver: H160,
        relayer: H160,
        fee: Uint,
    ) -> Self {
        Self {
            transaction_hash,
            block_number,
            pool: pool_by_addr(pool_address),
            receiver,
            relayer,
            fee,
        }
    }
}
