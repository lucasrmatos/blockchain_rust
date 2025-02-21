use std::collections::BTreeMap;
use num::traits::Zero;
use std::ops::AddAssign;

pub type AccId = String;
pub type BlockNumber = u64;
pub type Nonce = u32;  

#[derive(Debug)]
pub struct Pallet <BlockNumber, AccId, Nonce>{
    block_number: BlockNumber,
    nonce: BTreeMap<AccId, Nonce>
}


impl <BlockNumber, AccId, Nonce> Pallet <BlockNumber, AccId, Nonce>
where
    AccId: Ord + Clone,
    BlockNumber: Zero + Copy + AddAssign +From<u8>,
    Nonce: Zero + Copy + AddAssign + From<u8>
{

    pub fn new() -> Self{
        Pallet{
            block_number:BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> BlockNumber {
        self.block_number
    }

    pub fn increment_block_number( &mut self){
        self.block_number += BlockNumber::from(1);
    }

    pub fn inc_nonce( &mut self, account: &AccId){
        let nonce = * self.nonce.get(account).unwrap_or(&Nonce::zero()) + Nonce::from(1) ;
        self.nonce.insert(account.clone(), nonce);
    }    
    pub fn get_nonce( &self, account: &AccId) -> Option<&Nonce>{
        self.nonce.get(account)
    }

}