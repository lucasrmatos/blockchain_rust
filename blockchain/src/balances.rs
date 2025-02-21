use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};

pub type AccId = String;
pub type Balance = u128;


#[derive(Debug)]
pub struct Pallet <AccId, Balance> {
    balances: BTreeMap <AccId, Balance>

}

impl <AccId, Balance> Pallet <AccId, Balance> 
where 
    AccId: Ord + Clone + ToString,
    Balance: CheckedAdd + CheckedSub + Zero + Copy
{
    pub fn new() -> Self {
        Pallet{
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance( &mut self, account: &AccId, amount: Balance){
        self.balances.insert( account.clone(), amount);
        }

    pub fn balance (&self, account: &AccId) -> Balance{
        *self.balances.get( account).unwrap_or(&Balance::zero() )   
    }

    // #[allow(dead_code)]
    pub fn transfer( &mut self,
        caller: &AccId,
        to: &AccId, amount: Balance,) ->
        Result<(), &'static str>{

        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("insuficient Balance")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller.clone(), new_caller_balance);
        self.balances.insert(to.clone(), new_to_balance);
        Ok(())
    }
}