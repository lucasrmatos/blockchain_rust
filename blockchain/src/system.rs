use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>
}


impl Pallet{

    pub fn new() -> Self{
        Pallet{
            block_number:0,
            nonce: BTreeMap::new(),
        }
    }
    pub fn block_number(&self) -> u32{
        self.block_number
    }

    pub fn increment_block_number( &mut self){
        self.block_number += 1;
    }

    pub fn inc_nonce( &mut self, account: &String){
        let nonce = self.nonce.get(account).unwrap_or(&0)+1;
        self.nonce.insert(account.clone(), nonce);
    }    
    pub fn get_nonce( &self, account: &String) -> Option<&u32>{
        self.nonce.get(account)
    }

}