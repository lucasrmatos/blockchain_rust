use blockchain_rust::system::{Config, Pallet};

pub struct TestConfig;


impl Config for TestConfig{
    type AccId = String;
    type BlockNumber = u64 ;
    type Nonce = u32;
}

#[test]
fn init_system(){
    let mut system: Pallet<TestConfig>  = Pallet::new();

    assert_eq!(system.block_number(), 0);
    assert_eq!(system.get_nonce(&"lucas".to_string()), None);

    system.increment_block_number();
    assert_eq!(system.block_number(), 1);
        
    system.inc_nonce(&"lucas".to_string());
    assert_eq!(system.get_nonce(&"lucas".to_string()), Some(1).as_ref() );
    
}