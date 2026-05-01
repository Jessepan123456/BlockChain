use blockchain::blockchain::Chain;

#[test]
fn test_diff_change() {
    let mut chain = Chain::new("tenco".to_string(), 2);
    let res = chain.update_difficulty(3);
    assert_eq!(true, res);
}

#[test]
fn test_generateblock_work() {
    let mut chain = Chain::new("tenco".to_string(), 1);
    let res = chain.generate_new_block();
    assert_eq!(true, res);
}

#[test]
fn test_reward_change() {
    let mut chain = Chain::new("tenco".to_string(), 1);
    let res = chain.update_reward(100.0);
    assert_eq!(true, res);
}

#[test]
fn test_tran_change() {
    let mut chain = Chain::new("tenco".to_string(), 1);
    let res = chain.new_transaction("brock".to_string(), "tenco".to_string(), 100.0);
    assert_eq!(true, res);
    let amount = chain.get_transaction("tenco");
    assert_eq!(100.0, amount);
}

#[test]
fn test_blockamount_same(){
    let mut chain = Chain::new("tenco".to_string(), 1);
    let count = chain.block_count();
    assert_eq!(1, count);
}