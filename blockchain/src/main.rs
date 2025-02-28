mod balances;
mod system;
mod support;

use support::Dispatch;
use system::Config as SystemConfig;
use balances::Config as BalancesConfig;

mod types{
    pub type Balance = u128;  
    pub type AccId = String;
    pub type BlockNumber = u64;
    pub type Nonce = u32; 
    pub type Extrinsic = crate::support::Extrinsic<AccId, crate::RuntimeCall>; 
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

pub enum RuntimeCall{
    BalancesTransfer{ to: types::AccId, value: types::Balance  },
    ZeroBalance
}

#[derive(Debug)]
pub struct MainConfig;

impl SystemConfig for MainConfig{
    type AccId = String;
    type BlockNumber = u64 ;
    type Nonce = u32;
}

impl BalancesConfig for MainConfig{
    // type AccId = String;
    type Balance = u128;
}


#[derive(Debug)]
pub struct Runtime {
    balances: balances::Pallet<MainConfig>,
    system: system::Pallet<MainConfig>,
}

impl Runtime {
    pub fn new () -> Self{
        Runtime {
            balances: balances::Pallet::new(),
            system: system::Pallet::new(),
        }
    }

    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult{
        self.system.increment_block_number();
        if block.header.block_number != self.system.block_number(){
            return Err("Block number mismatch");
        }
        for(i, support::Extrinsic{caller, call})
        in block.extrinsics.into_iter().enumerate(){
            self.system.inc_nonce(&caller);
    
            let res = self.dispatch(caller, call).map_err(|e|  {
                format!("Error in block {}: extrinsic {}: {}", block.header.block_number, i ,e)
            }); 

            if let Err(e) = res{
                print!("Error in block {}: {}", block.header.block_number, e);
            }   
        }
        
        Ok(())
    }

}

impl crate::support::Dispatch for Runtime{
    type Caller =  <MainConfig as system::Config>::AccId;
    type Call = RuntimeCall; 

    fn dispatch(
        &mut self,
         caller: Self::Caller,
         runtime_call: Self::Call,)
           -> support::DispatchResult {
        match runtime_call{
            RuntimeCall::BalancesTransfer { to, value} => {
                self.balances.transfer(&caller, &to, value)
            },
            RuntimeCall::ZeroBalance =>{
                self.balances.set_balance(&caller, 0);
                Ok(())
            }
        }
    }
}



fn main() {
    // Crie uma nova instância do Runtime.
	// Ele irá instanciar com todos os módulos que usa.
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	let bob = "bob".to_string();
	let charlie = "charlie".to_string();

	// Inicialize o sistema com algum saldo inicial.
	runtime.balances.set_balance(&alice, 100);

	// Inicie a emulação de um bloco
	// runtime.system.increment_block_number();
	// assert_eq!(runtime.system.block_number(), 1);

    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::BalancesTransfer { to: "bob".to_string(), value: 69 },
            },
            support::Extrinsic {
                caller: "alice".to_string(),
                call: RuntimeCall::BalancesTransfer { to: "charlie".to_string(), value: 2 },
            },
        ],
    };

    runtime.execute_block(block_1).expect("invalid block");

    println!("{:#?}", runtime);

	// primeira transação
	runtime.system.inc_nonce(&alice);
	let _res = runtime
		.balances
		.transfer(&alice.clone(), &bob, 30)
		.map_err(|e| eprintln!("{}", e));

	// segunda transação
	runtime.system.inc_nonce(&alice);
	let _res = runtime.balances.transfer(&alice, &charlie, 20).map_err(|e| eprintln!("{}", e));


// let runtime =  Runtime::new();
//    println!("Hello, world!");
/*
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

println!("{:#?}", runtime);
println!("CABO");
*/


}
