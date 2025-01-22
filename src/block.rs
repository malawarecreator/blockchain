

use crate::hash::{self, calculate_hash};

pub struct Block {
    pub  data: String,
    pub hash: u64,
    pub previoushash: u64, 

}

impl Block {
    pub fn init(&mut self, prevhash: u64) {
        self.hash = calculate_hash(&self.data);
        self.previoushash = prevhash;


    } 
    pub fn print(self) {
        println!("data {}, hash: {}, prevhash: {}", self.data, self.hash, self.previoushash);

    }
    pub fn destroy(&mut self) {
        println!("goodbye");
        self.data = "dead".to_string();
        self.hash = 0;
        self.previoushash = 0;
    }


}