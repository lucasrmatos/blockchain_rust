use std::collections::BTreeMap;
use num::traits::Zero;
use std::ops::AddAssign;


pub trait Config {
    type AccId: Ord + Clone;
    type BlockNumber: Zero + Copy + AddAssign +From<u8>;
    type Nonce: Zero + Copy + AddAssign + From<u8>;
}

#[derive(Debug)]
pub struct Pallet <T: Config>{
    block_number: T::BlockNumber,
    nonce: BTreeMap<T::AccId, T::Nonce>
}


impl <T:Config> Pallet <T>
{

    pub fn new() -> Self{
        Pallet{
            block_number:T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    pub fn increment_block_number( &mut self){
        self.block_number += T::BlockNumber::from(1);
    }

    pub fn inc_nonce( &mut self, account: &T::AccId){
        let nonce = * self.nonce.get(account).unwrap_or(&T::Nonce::zero()) + T::Nonce::from(1) ;
        self.nonce.insert(account.clone(), nonce);
    }    
    /* 
    pub fn get_nonce( &self, account: &T::AccId) -> Option<&T::Nonce>{
        self.nonce.get(account)
    }
    */

}