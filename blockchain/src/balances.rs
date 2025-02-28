use std::collections::BTreeMap;
use num::traits::{CheckedAdd, CheckedSub, Zero};
use crate::support::DispatchResult;

/*
pub type AccId = String;
pub type Balance = u128;
 */

pub trait Config: crate::system::Config {
    // type AccId: Ord + Clone + ToString;
    type Balance: CheckedAdd + CheckedSub + Zero + Copy;
}

#[derive(Debug)]
pub struct Pallet <T: Config> {
    balances: BTreeMap <T::AccId, T::Balance>

}

impl <T: Config> Pallet <T> 
{
    pub fn new() -> Self {
        Pallet{
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance( &mut self, account: &T::AccId, amount: T::Balance){
        self.balances.insert( account.clone(), amount);
        }

    pub fn balance (&self, account: &T::AccId) -> T::Balance{
        *self.balances.get( account).unwrap_or(&T::Balance::zero() )   
    }

    // #[allow(dead_code)]
    pub fn transfer( &mut self,
        caller: &T::AccId,
        to: &T::AccId, amount: T::Balance,)
        ->  DispatchResult{

        let caller_balance = self.balance(caller);
        let to_balance = self.balance(to);

        let new_caller_balance = caller_balance.checked_sub(&amount).ok_or("insuficient Balance")?;
        let new_to_balance = to_balance.checked_add(&amount).ok_or("Overflow")?;

        self.balances.insert(caller.clone(), new_caller_balance);
        self.balances.insert(to.clone(), new_to_balance);
        Ok(())
    }
}