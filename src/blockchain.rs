extern crate time;  //Timer
extern crate serde; //Let you serialize 
extern crate serde_json; //serialize into JSON
extern crate sha2; // Some hashing algorithm

use serde_derive::Serialize;
use sha2::{Sha256, Digest};
use std::fmt::Write; //Write strings
use std::time::{SystemTime, UNIX_EPOCH};

//Represent a single transfer of value
#[derive(Debug, Clone, Serialize)]
// Debug let you use {:?}
// Clone let you use duplicate transactions
// Serialize -> convert into JSON
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

//Basically the blocking blocks that holds all the metadata
#[derive(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64, //Time
    nonce: u32, // Number miner (Basically the hash that you keep
    // on changing until you found what hash your looking for)
    pre_hash: String, // previous block
    merkle: String, // Merkle root of all transactions
    difficulty: u32, //Mining difficulty
}

//The actual full block
#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader, //Name or metadata
    count: u32, // Amount of transactions
    transaction: Vec<Transaction> // list
}

//The chains that connects the blocks
#[derive(Serialize)]
pub struct Chain {
    chain: Vec<Block>, //list of all blocks
    curr_trans: Vec<Transaction>, //pending transactions
    difficulty: u32, //mining difficulty
    miner_addr: String, // address that receives mining reward
    reward: f32, //Amount from mining
}

impl Chain {
    //Creating the initial Chain
    pub fn new(miner_addr: String, difficulty: u32) -> Chain {
        let mut chain = Chain { //Default setting
            chain: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        chain.generate_new_block(); //Generate the new block
        chain

    }

    pub fn block_count(&self) -> usize{
        return self.chain.len();
    }

    //Add a Pending transaction
    //Not in the blockchain yet it still pending
    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        if amount != 0.0 {
                let sender = sender.trim().to_string();
                let receiver = receiver.trim().to_string();
                self.curr_trans.push(Transaction{
                sender,
                receiver,
                amount,
            });

            true
        }
        else {
            false
        }
    }

    pub fn get_transaction(&self, address : &str) -> f32 {
        let mut balance = 0.0;

        for block in &self.chain{
            for transaction in &block.transaction {
                if transaction.receiver == address
                {
                    balance += transaction.amount;
                }
                if transaction.sender == address 
                {
                    if balance - transaction.amount < 0.0 {
                        return 0.0
                    }
                    balance -= transaction.amount
                }
            }
        }
        return balance
    }

    pub fn get_sender(&self) -> Vec<&String> {
        let mut list: Vec<&String> = Vec::new();
        let mut sender;

        for block in &self.chain {
            for transaction in &block.transaction {
                sender = &transaction.sender;
                list.push(sender);
            }
        }
        return list.to_vec();
    }

    pub fn get_receiver(&self) -> Vec<&String> {
        let mut list: Vec<&String> = Vec::new();
        let mut receiver;

        for block in &self.chain {
            for transaction in &block.transaction {
                receiver = &transaction.receiver;
                list.push(receiver);
            }
        }
        return list.to_vec();
    }

    //Get the hash from the most recent block
    pub fn last_hash(&self) -> String {
        let block = match self.chain.last() {
            //Take the block
            Some(block) => block,
            //If first created we don't have anything, so we just fill it up with something
            //It mean if the chain is empty
            None => return String::from_utf8(vec![48; 64]).unwrap()
        };
        //Hashes it and return it
        Chain::hash(&block.header)
    }

    //Updates the difficulty
    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    //Update the reward 
    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate_new_block(&mut self) -> bool {
        //Set up the metadata for the block
        let header = Blockheader {
            timestamp: SystemTime::now() .duration_since(UNIX_EPOCH) .unwrap() .as_secs() as i64,
            nonce: 0,
            pre_hash: self.last_hash(),
            merkle: String::new(),
            difficulty: self.difficulty,
        };

        //Mining reward
        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward
        };

        //Build the block with the new reward transaction from mining
        let mut block = Block {
            header,
            count: 0,
            transaction: vec![]
        };

        //Push all that new metadata into the chain
        block.transaction.push(reward_trans);
        block.transaction.append(&mut self.curr_trans);
        block.count = block.transaction.len() as u32;
        block.header.merkle = Chain::get_merkle(block.transaction.clone());
        Chain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.chain.push(block);
        true
    }

    //Merkle root
    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        //Hashes each transaction
        for t in &curr_trans {
            let hash = Chain::hash(t);
            merkle.push(hash);
        }

        //If odd number of hashes -> duplicate last one so it even
        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        //Repeatedly combine pairs until only one hash remains
        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Chain::hash(&h1);
            merkle.push(nh);
        }
        //Merkle root
        merkle.pop().unwrap()
    }

    //Use the difficulty for mining 
    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            };
        }
    }

    // Use to serialize
    // Used for hashing 
    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        //Convert into JSON
        let input = serde_json::to_string(&item).unwrap();
        //Feed the JSON into SHA-256
        let mut hasher = Sha256::default();
        hasher.update(input.as_bytes());
        //Finalize the hash
        let res = hasher.finalize();
        let vec_res = res.to_vec();

        //convert bytes to hex string
        Chain::hex_to_string(vec_res.as_slice())
    }

    //Looping through eachc byte and formats it as hex
    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for b in vec_res {
            write!(&mut s, "{:x}", b).expect("unable to write");
        }
        s
    }
}