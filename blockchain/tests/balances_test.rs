use blockchain::balances::Pallet;

#[test]
pub fn init_balances() {
    let mut balances = Pallet::new();

    assert_eq!(balances.balance(&"lucas".to_string()), 0);
    balances.set_balance(&"lucas".to_string(), 10);

    assert_eq!(balances.balance(&"lucas".to_string()), 10);
    assert_eq!(balances.balance(&"vini".to_string()), 0);
}

#[test]
fn transfer_balance() {
    let mut balances = Pallet::new();
    /* // 1.4
    balances.set_balance(&"lucas".to_string(), 10);
    assert_eq!(balances.transfer(&"lucas".to_string(), &"vini".to_string(), 10), Ok(()));

    assert_eq!(balances.balance(&"lucas".to_string()), 0);
    assert_eq!(balances.balance(&"vini".to_string()), 10);
    */

    // 1.5 

    // lucas n tem nada -> erro
    assert_eq!(balances.transfer(&"lucas".to_string(), &"pedro".to_string() , 10), Err("insuficient balance") );

    balances.set_balance(&"lucas".to_string(), 10);
    
    assert_eq!(balances.transfer(&"lucas".to_string(), &"pedro".to_string() , 4), Ok(()) );

    assert_eq!(balances.balance(&"lucas".to_string() ), 6 );
    assert_eq!(balances.balance(&"pedro".to_string() ), 4 );

    balances.set_balance(&"pedro".to_string(), u128::MAX);
    assert_eq!(balances.transfer(&"lucas".to_string(), &"pedro".to_string() , 1), Err("Overflow") );


    }