// use blockchain::balances::Pallet;
// use blockchain::system::Pallet;

mod system;
mod balances;

pub struct Runtime {
    balances: balances::Pallet,
    system: system::Pallet,
}

impl Runtime {
    pub fn new () -> Self{
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }
}


fn main() {

// let runtime =  Runtime::new();
//    println!("Hello, world!");

let mut runtime = Runtime::new();
let lucas = "lucas".to_string();
let pedro = "pedro".to_string();
let tainara = "tainara".to_string();

runtime.balances.set_balance(&tainara, 100);
runtime.system.increment_block_number();
assert!(runtime.system.block_number() == 1);

runtime.system.inc_nonce(&tainara);

let _res = runtime.balances.
transfer(&tainara, &lucas, 30)
.map_err( |e| println!("{}", e));

runtime.system.inc_nonce(&tainara);

let _res = runtime.balances.
transfer(&tainara, &pedro, 20)
.map_err( |e| println!("{}", e));

println!("CABO");

}
