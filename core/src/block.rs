use super::header::Header;
use super::transaction::Transaction;
use super::Error;
use bigint::H256;
use merkle_root::*;
use nervos_protocol;

#[derive(Clone, Serialize, Deserialize, PartialEq, Default, Debug)]
pub struct Block {
    pub header: Header,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn hash(&self) -> H256 {
        self.header.hash()
    }

    pub fn is_genesis(&self) -> bool {
        self.header.is_genesis()
    }

    //TODO: move to verification
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }

    //TODO: move to verification
    pub fn check_txs_root(&self) -> Result<(), Error> {
        let txs_hash: Vec<H256> = self.transactions.iter().map(|t| t.hash()).collect();
        let txs_root = merkle_root(txs_hash.as_slice());
        if txs_root == self.header.txs_commit {
            Ok(())
        } else {
            Err(Error::InvalidTransactionsRoot(
                self.header.txs_commit,
                txs_root,
            ))
        }
    }

    pub fn new(header: Header, transactions: Vec<Transaction>) -> Block {
        Block {
            header,
            transactions,
        }
    }
}

impl<'a> From<&'a nervos_protocol::Block> for Block {
    fn from(b: &'a nervos_protocol::Block) -> Self {
        Block {
            header: b.get_header().into(),
            transactions: b.get_transactions().iter().map(|t| t.into()).collect(),
        }
    }
}

impl<'a> From<&'a Block> for nervos_protocol::Block {
    fn from(b: &'a Block) -> Self {
        let mut block = nervos_protocol::Block::new();
        block.set_header(b.header().into());
        let transactions = b.transactions.iter().map(|t| t.into()).collect();
        block.set_transactions(transactions);
        block
    }
}

#[cfg(test)]
mod tests {}
