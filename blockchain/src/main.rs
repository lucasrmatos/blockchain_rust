use blockchain::balances::Pallet;


fn main() {
    println!("Hello, world!");
    let mut pallet: Pallet = Pallet::new();
    pallet.set_balance(&"lucas".to_string() ,2 );

    let balance = pallet.balance(&"lucas".to_string());
    println!("balanço {}", balance);
}
