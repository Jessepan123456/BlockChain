extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;
use std::fs::File;

mod blockchain;
fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut miner_addr).unwrap();
    print!("Diifficulty: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut difficulty).unwrap();
    let diff = difficulty.trim().parse::<u32>().expect("we need an integer");
    println!("generating genesis block: ");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("Menu");
        println!("1) Transaction");
        println!("2) Mine block");
        println!("3) Change Difficulty");
        println!("4) Change Reward");
        println!("5) Save to file");
        println!("0) Exit");
        io::stdout().flush().unwrap();
        choice.clear();
        io::stdin().read_line(&mut choice).unwrap();
        println!("");

        match choice.trim().parse().unwrap() {
            0 => 
            {
                println!("Exiting...");
                process::exit(0);
            }
            1 =>
            {
                println!("0) Get Transaction(mine before using this)");
                println!("1) Get List of Senders");
                println!("2) Get List of Receiver");
                println!("3) New Transaction");
                io::stdout().flush().unwrap();
                choice.clear();
                io::stdin().read_line(&mut choice).unwrap();
                match choice.trim().parse().unwrap() {
                    0 =>
                    {
                        let mut address = String::new();

                        print!("enter address: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut address).unwrap();

                        let res = chain.get_transaction(address.trim());
                        println!("{}", res);
                    }
                    1 =>
                    {
                        let root = "Root";
                        let list = chain.get_sender();
                        println!("List Of Senders:");
                        for name in &list
                        {
                            if name.as_str() != root
                            {
                                println!("{}", name);
                            }
                        }
                    }
                    2 =>
                    {
                        let list = chain.get_receiver();
                        println!("List Of Receiver:");
                        for name in &list
                        {
                            println!("{}", name);
                        }
                    }
                    3 => 
                    {
                        let mut sender = String::new();
                        let mut receiver = String::new();
                        let mut amount = String::new();

                        print!("enter sender address: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut sender).unwrap();
                        print!("enter receiver address: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut receiver).unwrap();
                        print!("enter amount: ");
                        io::stdout().flush().unwrap();
                        io::stdin().read_line(&mut amount).unwrap();

                        let res = chain.new_transaction(sender.trim().to_string(),
                                                receiver.trim().to_string(),
                                                amount.trim().parse().unwrap());

                        match res {
                            true => println!("transaction added"),
                            false => println!("transaction failed"),
                }
                    }
                    _ => println!("invalid option choice")
                }
            }
            2 =>
            {
                println!("Generating block");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => 
            {
                let mut new_diff = String::new();
                print!("enter new difficulty: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_diff).unwrap();
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Updated Difficulty"),
                    false => println!("Failed Updated Difficulty"),
                }
            }
            4 =>
            {
                let mut new_reward = String::new();
                print!("Enter new reward: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut new_reward).unwrap();
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Updated reward"),
                    false => println!("Failed Update reward"),
                }
            }
            5 =>
            {
                let mut filename = String::new();
                print!("Enter a filename with .txt at the end: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut filename).unwrap();
                let f = filename.trim();
                
                let decode = serde_json::to_string(&chain).unwrap();
                let mut file = File::create(f).unwrap();
                file.write_all(decode.as_bytes()).unwrap();
            }
            _ => println!("\tinvalid option please retry\t")
        }
    }
}
