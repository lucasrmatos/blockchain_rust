use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<String, u128>

}

impl Pallet {
    pub fn new() -> Self {
        Pallet{
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance( &mut self, account: &String, amount: u128){
        self.balances.insert( account.to_string(), amount);
        }

    pub fn balance (&self, account: &String) -> u128{
        *self.balances.get( account).unwrap_or(&0)   
    }

    // #[allow(dead_code)]
    pub fn transfer( &mut self, caller: &String, to: &String, amount: u128,) -> Result<(), &'static str>{
        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("insuficient balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("Overflow")?;

        self.balances.insert(caller.to_string(), new_caller_balance);
        self.balances.insert(to.to_string(), new_to_balance);
        Ok(())
    }
}