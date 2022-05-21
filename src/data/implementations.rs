use super::{
    ESInternalTransaction, ESInternalTransactionStrings, ESNormalTransaction,
    ESNormalTransactionStrings, ESTransaction,
};
use hex::decode as hex_decode;
use std::error::Error;
use web3::types::{H160, H256};

impl ESTransaction for ESInternalTransaction {
    fn transaction_hash(&self) -> H256 {
        self.hash
    }
    fn transaction_value(&self) -> u128 {
        self.value
    }
    fn transaction_to(&self) -> Option<H160> {
        Some(self.to)
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

impl PartialEq for ESInternalTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.blockNumber == other.blockNumber
            && self.timeStamp == other.timeStamp
            && self.hash == other.hash
            && self.from == other.from
            && self.to == other.to
            && self.value == other.value
            && self.contractAddress == other.contractAddress
            && self.input == other.input
            && self.type_ == other.type_
            && self.gas == other.gas
            && self.gasUsed == other.gasUsed
            && self.traceId == other.traceId
            && self.isError == other.isError
            && self.errCode == other.errCode
    }
}

impl PartialEq for ESNormalTransaction {
    fn eq(&self, other: &Self) -> bool {
        self.blockNumber == other.blockNumber
            && self.timeStamp == other.timeStamp
            && self.hash == other.hash
            && self.from == other.from
            && self.to == other.to
            && self.value == other.value
            && self.contractAddress == other.contractAddress
            && self.input == other.input
            && self.gas == other.gas
            && self.gasUsed == other.gasUsed
            && self.isError == other.isError
            && self.txreceipt_status == other.txreceipt_status
            && self.transactionIndex == other.transactionIndex
            && self.blockHash == other.blockHash
            && self.blockNumber == other.blockNumber
            && self.confirmations == other.confirmations
            && self.cumulativeGasUsed == other.cumulativeGasUsed
            && self.nonce == other.nonce
            && self.gasPrice == other.gasPrice
    }
}

impl TryInto<ESInternalTransaction> for ESInternalTransactionStrings {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<ESInternalTransaction, Box<dyn Error>> {
        Ok(ESInternalTransaction {
            blockNumber: self.blockNumber.parse()?,
            timeStamp: self.timeStamp.parse()?,
            hash: self.hash.get(2..).unwrap().parse()?,
            from: self.from.get(2..).unwrap().parse()?,
            to: self.to.get(2..).unwrap().parse()?,
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
            type_: self.type_.parse()?,
            gas: self.gas.parse()?,
            gasUsed: self.gasUsed.parse()?,
            traceId: self.traceId,
            isError: self.isError.parse()?,
            errCode: if self.errCode.is_empty() {
                None
            } else {
                Some(self.errCode.parse()?)
            },
        })
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
