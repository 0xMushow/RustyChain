use crate::block::Block;

#[derive(Debug)]
pub struct Blockchain {
    pub blocks: Vec<Block>,
    pub difficulty: u64,
}

impl Blockchain {
    pub fn new(difficulty: u64) -> Self {
        let genesis_block = Block::new(0, String::from("0"), String::from("Genesis Block"));

        Blockchain {
            blocks: vec![genesis_block],
            difficulty,
        }
    }

    pub fn add_block(&mut self, data: String) {
        let index = self.blocks.len() as u64;
        let prev_hash = self.blocks.last().unwrap().hash.clone();

        let mut new_block = Block::new(index, prev_hash, data);
        self.mine_block(&mut new_block);
        self.blocks.push(new_block);
    }

    pub fn mine_block(&self, block: &mut Block) {
        let target_prefix = "0".repeat(self.difficulty as usize);

        while !block.calculate_hash().starts_with(&target_prefix) {
            block.nonce += 1;
        }
        block.hash = block.calculate_hash();
    }
}