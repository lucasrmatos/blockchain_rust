use blockchain_rust::system::{Pallet, BlockNumber, AccId, Nonce};

#[test]
fn init_system(){
    let mut system: blockchain_rust::system::Pallet<BlockNumber, String, i32>  = Pallet::new();

    assert_eq!(system.block_number(), 0);
    assert_eq!(system.get_nonce(&"lucas".to_string()), None);

    system.increment_block_number();
    assert_eq!(system.block_number(), 1);
        
    system.inc_nonce(&"lucas".to_string());
    assert_eq!(system.get_nonce(&"lucas".to_string()), Some(1).as_ref() );
    
}